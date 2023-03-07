use std::str::FromStr;
use json_value_merge::Merge;
use crate::types::{ChatCompletionChunk, ConversationResponse, Message, CompletionOptions,ResponsePart};
use eventsource_stream::{EventStream, Eventsource};
use futures_util::Stream;
use futures_util::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Method, Url,
};
use serde_json::json;
use uuid::Uuid;

/// Options for the ChatGPT client
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClientOptions {
    backend_api_url: Url,
}

impl ClientOptions {
    /// Sets the default backend API url. This is different from [`Self::with_api_url`] and defaults to https://chat.openai.com/backend-api
    pub fn with_backend_api_url(mut self, backend_url: Url) -> Self {
        self.backend_api_url = backend_url;
        self
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            backend_api_url: Url::from_str("https://api.openai.com/v1/chat/completions").unwrap(),
        }
    }
}

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
    options: ClientOptions,
    api_key: String,
}

impl ChatGPT {
    /// Constructs a new ChatGPT client with default client options
    pub fn new<S: Into<String>>(token: S) -> crate::Result<Self> {
        Self::with_options(token, ClientOptions::default())
    }

    /// Constructs a new ChatGPT client with the specified client options
    pub fn with_options<S: Into<String>>(token: S, options: ClientOptions) -> crate::Result<Self> {
        let token = token.into();
        let client = reqwest::ClientBuilder::new().build()?;
        Ok(Self {
            client,
            options,
            api_key: token,
        })
    }

    /// Sends a messages and gets ChatGPT response.
    ///
    /// Note that usually it takes the AI around ~10-30 seconds to respond because of how the backend API is implemented.
    /// Because of that, sometimes you might want to use [`Self::send_message_streaming()`]
    ///
    /// Example:
    /// ```rust
	/// # use chatgpt::types::Message;
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("OPENAI_SK").unwrap())?;
    /// let messages = vec![Message {
    ///     role: "user".to_owned(),
    ///     content: "Write me a simple sorting algorithm in Rust".to_owned(),
    /// }];
    /// let response: String = client.send_message(messages).await?;
    /// println!("{response}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message<S: Into<Vec<Message>>>(&self, message: S, options: CompletionOptions) -> crate::Result<String> {
        self.send_message_full(message, options)
            .await
            .map(|value| value.choices[0].message.content.to_owned())
    }

    /// Sends a message with parent message id and conversation id for conversations.
    ///
    /// Note that usually it takes the AI around ~10-30 seconds to respond because of how the backend API is implemented.
    /// Because of that, sometimes you might want to use [`Self::send_message_streaming()`]
    ///
    /// Example:
    /// ```rust
	/// # use chatgpt::types::Message;
    /// # use chatgpt::prelude::*;
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("OPENAI_SK").unwrap())?;
    /// # let messages = vec![Message {
    /// #    role: "user".to_owned(),
    /// #    content: "Write me a simple sorting algorithm in Rust".to_owned(),
    /// # }];
    /// # let response: ConversationResponse = client.send_message_full(messages).await?;
    /// # println!("{response:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_full<S: Into<Vec<Message>>>(
        &self,
        message: S,
		options: CompletionOptions,
    ) -> crate::Result<ConversationResponse> {
        let message = message.into();
		let mut body = serde_json::to_value(options)?;
		

		body["model"] = serde_json::Value::String(String::from("gpt-3.5-turbo"));
		body["messages"] = serde_json::to_value(message)?;
        let resp = self
            .client
            .request(Method::POST, self.options.backend_api_url.clone())
            .header("Content-Type", "application/json".to_owned())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;
        let resp = dbg!(resp.text().await)?;
        let res: ConversationResponse = serde_json::from_str(&resp).map_err(|_e| crate::err::Error::ApiError(resp))?;
        Ok(res)
    }

    /// Sends a message with full configuration and returns a stream of gradually finishing message
    ///
    /// Example:
    /// ```rust
	/// # use chatgpt::types::Message;
    /// # use chatgpt::types::ResponsePart;
    /// # use chatgpt::client::ChatGPT;
    /// # use futures_util::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("OPENAI_SK").unwrap())?;
    /// let messages = vec![Message {
    ///     role: "user".to_owned(),
    ///     content: "Write me a simple sorting algorithm in Rust".to_owned(),
    /// }];
    /// let mut stream = client.send_message_streaming(messages).await?;
    /// while let Some(message) = stream.next().await {
    ///     match message? {
    ///         ResponsePart::Chunk(data) => {
    ///             println!("Got part of data: {data:?}");
    ///         }
    ///         ResponsePart::Done => {
    ///             println!("Data processing finished! Response")
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_streaming<S: Into<Vec<Message>>>(
        &self,
        message: S,
    ) -> crate::Result<impl Stream<Item = crate::Result<ResponsePart>>> {
        let stream = self.acquire_response_stream(message.into()).await?;

        Ok(stream.map(move |part| {
            let chunk = part?.data;
            dbg!(&chunk);
            if chunk == "[DONE]" {
                crate::Result::Ok(ResponsePart::Done)
            } else {
				let data: ChatCompletionChunk = serde_json::from_str(&chunk)?;
                crate::Result::Ok(ResponsePart::Chunk(data))
            }
        }))
    }

    async fn acquire_response_stream(
        &self,
        messages: Vec<Message>,
    ) -> crate::Result<EventStream<impl Stream<Item = reqwest::Result<bytes::Bytes>>>> {
        let body = json!({
            "model": "gpt-3.5-turbo",
            "stream": true,
            "messages": messages
            // "parent_message_id": parent_message_id.unwrap_or_else(Uuid::new_v4),
        });
        Ok(self
            .client
            .request(Method::POST, self.options.backend_api_url.clone())
            .header("Content-Type", "application/json".to_owned())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?
            .bytes_stream()
            .eventsource())
    }
}
