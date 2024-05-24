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

/// MessageDeltaContentImageFileObject : References an image [File](/docs/api-reference/files) in the content of a message.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentImageFileObject {
    /// The index of the content part in the message.
    #[serde(rename = "index")]
    pub index: i32,
    /// Always `image_file`.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "image_file", skip_serializing_if = "Option::is_none")]
    pub image_file: Option<Box<models::MessageDeltaContentImageFileObjectImageFile>>,
}

impl MessageDeltaContentImageFileObject {
    /// References an image [File](/docs/api-reference/files) in the content of a message.
    pub fn new(index: i32, r#type: Type) -> MessageDeltaContentImageFileObject {
        MessageDeltaContentImageFileObject {
            index,
            r#type,
            image_file: None,
        }
    }
}
/// Always `image_file`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "image_file")]
    ImageFile,
}

impl Default for Type {
    fn default() -> Type {
        Self::ImageFile
    }
}

