/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see https://platform.openai.com/docs/api-reference for more details.
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessage {
    ChatCompletionRequestSystemMessage(Box<models::ChatCompletionRequestSystemMessage>),
    ChatCompletionRequestUserMessage(Box<models::ChatCompletionRequestUserMessage>),
    ChatCompletionRequestAssistantMessage(Box<models::ChatCompletionRequestAssistantMessage>),
    ChatCompletionRequestToolMessage(Box<models::ChatCompletionRequestToolMessage>),
    ChatCompletionRequestFunctionMessage(Box<models::ChatCompletionRequestFunctionMessage>),
}

impl Default for ChatCompletionRequestMessage {
    fn default() -> Self {
        Self::ChatCompletionRequestSystemMessage(Default::default())
    }
}
/// The role of the messages author, in this case `function`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "function")]
    Function,
}

impl Default for Role {
    fn default() -> Role {
        Self::Function
    }
}

