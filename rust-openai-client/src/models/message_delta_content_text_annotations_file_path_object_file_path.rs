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
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    /// The ID of the file that was generated.
    #[serde(rename = "file_id", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

impl MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    pub fn new() -> MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
        MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
            file_id: None,
        }
    }
}
