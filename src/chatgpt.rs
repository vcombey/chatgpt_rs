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
    use crate::{client::ChatGPT, types::Message};
    use futures_util::StreamExt;
    #[tokio::test]
    async fn test_client() {
        let token = std::env::var("OPENAI_SK").unwrap();
        let _client = ChatGPT::new(&token).unwrap();
    }
    #[tokio::test]
    async fn test_message_streaming() -> crate::Result<()> {
        let org = "org-xzE2hBner5ZwF3wAyvMytmsd".to_string();
        let token = std::env::var("OPENAI_SK").unwrap();
        let messages = vec![Message {
            role: "user".to_owned(),
            content: Some("Write me a simple sorting algorithm in Rust".to_owned()),
            function_call: None,
        }];
        let client = ChatGPT::new(&token)?;
        let mut stream = client
            .send_message_streaming(messages, Default::default(), org)
            .await?;
        while let Some(chunk) = stream.next().await {
            dbg!(&chunk);
        }
        panic!("panic");
        Ok(())
    }
    #[tokio::test]
    async fn test_message() -> crate::Result<()> {
        let org = "org-xzE2hBner5ZwF3wAyvMytmsd".to_string();
        let token = std::env::var("OPENAI_SK").unwrap();
        // std::env::var("SESSION_TOKEN").unwrap();
        let messages = vec![Message {
            role: "user".to_owned(),
            content: Some("Write me a simple sorting algorithm in Rust".to_owned()),
            function_call: None,
        }];
        let client = ChatGPT::new(&token)?;
        let response = client
            .send_message_full(messages, Default::default(), org)
            .await?;
        println!("{:?}", response);
        Ok(())
    }
}
