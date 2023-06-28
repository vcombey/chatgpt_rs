use core::f32;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// A response that is received on the conversation endpoint
#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct ConversationResponse {
    // {
    //   "id": "chatcmpl-123",
    //   "object": "chat.completion",
    //   "created": 1677652288,
    //   "choices": [{
    //     "index": 0,
    //     "message": {
    //       "role": "assistant",
    //       "content": "\n\nHello there, how may I assist you today?",
    //     },
    //     "finish_reason": "stop"
    //   }],
    //   "usage": {
    //     "prompt_tokens": 9,
    //     "completion_tokens": 12,
    //     "total_tokens": 21
    //   }
    // }
    /// Unique ID of the message
    pub id: String,
    /// Content of this message
    pub object: String,
    /// Kind of sender. Either AI or user
    pub created: u64,
    /// The user that sent this message
    pub choices: Vec<ConversationChoice>,
    pub usage: Usage,
}

/// The message that the user or the AI sent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Usage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

/// The message that the user or the AI sent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ConversationChoice {
    pub index: u64,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Message {
    pub content: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
}

/// Kind of sender
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// A user sent this message
    User,
    /// An AI sent this message
    Assistant,
}

/// Part of a mapped response returned from the [`ChatGPT::send_message_streaming()`](`chatgpt::client::ChatGPT::send_message_streaming()`) method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ResponsePart {
    /// Got a chunk of response containing unfinished message response
    Chunk(ChatCompletionChunk),
    /// Got an indication that the final response was returned
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ChatCompletionChunk {
    id: String,
    created: u64,
    model: String,
    pub choices: Vec<ChoiceChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ChoiceChunk {
    pub delta: DeltaChunk,
    index: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct DeltaChunk {
    pub content: Option<String>,
    role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CompletionOptions {
    ///model
    ///string
    ///Required
    ///
    ///ID of the model to use. Currently, only gpt-3.5-turbo and gpt-3.5-turbo-0301 are supported.
    ///messages
    ///array
    ///Required
    pub model: Option<String>,
    ///
    ///The messages to generate chat completions for, in the chat format.
    ///temperature
    ///number
    ///Optional
    ///Defaults to 1
    ///
    ///What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    ///
    ///We generally recommend altering this or top_p but not both.
    ///top_p
    ///number
    ///Optional
    ///Defaults to 1
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    ///
    ///An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    ///We generally recommend altering this or temperature but not both.
    ///n
    ///integer
    ///Optional
    ///Defaults to 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<f32>,
    ///
    ///How many chat completion choices to generate for each input message.
    ///stream
    ///boolean
    ///Optional
    ///Defaults to false
    ///
    ///If set, partial message deltas will be sent, like in ChatGPT. Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    ///stop
    ///string or array
    ///Optional
    ///Defaults to null
    ///
    ///Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    ///max_tokens
    ///integer
    ///Optional
    ///Defaults to inf
    ///
    ///The maximum number of tokens allowed for the generated answer. By default, the number of tokens the model can return will be (4096 - prompt tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    ///presence_penalty
    ///number
    ///Optional
    ///Defaults to 0
    ///
    ///Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    ///See more information about frequency and presence penalties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<usize>,
    ///frequency_penalty
    ///number
    ///Optional
    ///Defaults to 0
    ///
    ///Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<usize>,
    ///user
    ///string
    ///Optional
    ///
    ///A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. Learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<isize>,
}
