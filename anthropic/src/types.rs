//! Module for types used in the API.
use std::pin::Pin;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

use crate::error::AnthropicError;
use crate::DEFAULT_MODEL;

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "AnthropicError"))]
pub struct CreateMessageRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub system: Option<String>,
    pub max_tokens: i32,
    pub stop_sequences: Option<Vec<String>>,
    #[builder(default = "false")]
    pub stream: bool,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String, // Always "message"
    pub role: String, // Always "assistant"
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageSource>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "media_type")]
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CreateMessageResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String, // Always "message"
    pub role: String, // Always "assistant"
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "message_start")]
    MessageStart {
        message: Message,
    },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: usize,
        delta: ContentDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop {
        index: usize,
    },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: MessageDelta,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "error")]
    Error {
        error: ErrorData,
    },
    // Fallback for unknown events
    Unknown {},
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ContentDelta {
    #[serde(rename = "type")]
    pub delta_type: String, // Currently, can be "text_delta"
    pub text: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ErrorData {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Clone, Serialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "AnthropicError"))]
pub struct CompleteRequest {
    /// The prompt to complete.
    pub prompt: String,
    /// The model to use.
    #[builder(default = "DEFAULT_MODEL.to_string()")]
    pub model: String,
    /// The number of tokens to sample.
    pub max_tokens_to_sample: usize,
    /// The stop sequences to use.
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to incrementally stream the response.
    #[builder(default = "false")]
    pub stream: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CompleteResponse {
    pub completion: String,
    pub stop_reason: Option<StopReason>,
}

/// Parsed server side events stream until a [StopReason::StopSequence] is received from server.
pub type CreateMessageResponseStream = Pin<Box<dyn Stream<Item = Result<StreamEvent, AnthropicError>> + Send>>;

/// Parsed server side events stream until a [StopReason::StopSequence] is received from server.
pub type CompleteResponseStream = Pin<Box<dyn Stream<Item = Result<CompleteResponse, AnthropicError>> + Send>>;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    MaxTokens,
    StopSequence,
}
