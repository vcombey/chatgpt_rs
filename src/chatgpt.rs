#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the ChatGPT client
pub mod client;
/// This module contains the errors related to the API
pub mod err;
/// The prelude module. Import everything from it to get the necessary elements from this library
pub mod prelude;
/// Types returned from the API and sent to it
pub mod types;

/// Result that is returned from most ChatGPT functions
pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
pub mod test {
    use crate::{client::ChatGPT, types::Message, types::ResponsePart};
    use futures_util::StreamExt;
    #[tokio::test]
    async fn test_client() {
        let token = std::env::var("OPENAI_SK").unwrap();
        let client = ChatGPT::new(&token).unwrap();
    }

    #[tokio::test]
    async fn test_message() -> crate::Result<()> {
        let token = std::env::var("OPENAI_SK").unwrap();
        // std::env::var("SESSION_TOKEN").unwrap();
        let messages = vec![Message {
            role: "user".to_owned(),
            content: "Write me a simple sorting algorithm in Rust".to_owned(),
        }];
        let client = ChatGPT::new(&token)?;
        let response = client.send_message_full(messages, Default::default()).await?;
        println!("{:?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_streaming() -> crate::Result<()> {
        let token = std::env::var("OPENAI_SK").unwrap();
        let client = ChatGPT::new(&token)?;
        let messages = vec![Message {
            role: "user".to_owned(),
            content: "Write me a simple sorting algorithm in Rust".to_owned(),
        }];
        let mut stream = client.send_message_streaming(messages).await?;
        while let Some(element) = stream.next().await {
            let element = element?;
            println!("{element:#?}")
        }
        Ok(())
    }

}
