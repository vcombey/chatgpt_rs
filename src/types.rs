use core::f32;

use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
