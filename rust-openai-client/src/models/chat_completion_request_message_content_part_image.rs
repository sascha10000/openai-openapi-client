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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartImage {
    /// The type of the content part.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "image_url")]
    pub image_url: Box<models::ChatCompletionRequestMessageContentPartImageImageUrl>,
}

impl ChatCompletionRequestMessageContentPartImage {
    pub fn new(r#type: Type, image_url: models::ChatCompletionRequestMessageContentPartImageImageUrl) -> ChatCompletionRequestMessageContentPartImage {
        ChatCompletionRequestMessageContentPartImage {
            r#type,
            image_url: Box::new(image_url),
        }
    }
}
/// The type of the content part.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "image_url")]
    ImageUrl,
}

impl Default for Type {
    fn default() -> Type {
        Self::ImageUrl
    }
}

