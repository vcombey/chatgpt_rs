#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the ChatGPT client
pub mod client;
/// This module contains all the conversation logic
pub mod converse;
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
        let mut client = ChatGPT::new(&token).unwrap();
    }

    #[tokio::test]
    async fn test_message() -> crate::Result<()> {
        let token = std::env::var("OPENAI_SK").unwrap();
        // std::env::var("SESSION_TOKEN").unwrap();
        let messages = vec![Message {
            role: "user".to_owned(),
            content: "Write me a simple sorting algorithm in Rust".to_owned(),
        }];
        let mut client = ChatGPT::new(&token)?;
        let response = client.send_message_full(messages).await?;
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

    #[tokio::test]
    async fn test_conversations() -> crate::Result<()> {
        let token = std::env::var("OPENAI_SK").unwrap();
        let mut client = ChatGPT::new(&token)?;
        // let mut conversation = client.new_conversation();
        // let response = conversation
        //     .send_message(&client, "Write a simple sorting algorithm in Rust")
        //     .await?;
        // println!("{response}");
        // let response = conversation
        //     .send_message(&client, "Now can you rewrite it in Kotlin?")
        //     .await?;
        // println!("{response}");
        Ok(())
    }

    #[tokio::test]
    async fn test_conversations_streaming() -> crate::Result<()> {
        let token = std::env::var("OPENAI_SK").unwrap();
        let mut client = ChatGPT::new(&token)?;
        // let mut conversation = client.new_conversation();
        // let response = conversation
        //     .send_message(&client, "Write a simple sorting algorithm in Rust")
        //     .await?;
        // println!("{response}");
        // let mut stream = conversation
        //     .send_message_streaming(&client, "Now can you rewrite it in Kotlin?")
        //     .await?;
        // while let Some(part) = stream.next().await {
        //     let response = part?;
        //     match response {
        //         ResponsePart::Processing(data) => {
        //             println!("{}", data.message.content.parts[0]);
        //         }
        //         _ => continue,
        //     }
        // }
        Ok(())
    }
}
