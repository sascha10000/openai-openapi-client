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

/// BatchRequestInput : The per-line object of the batch input file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestInput {
    /// A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch.
    #[serde(rename = "custom_id", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// The HTTP method to be used for the request. Currently only `POST` is supported.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /// The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl BatchRequestInput {
    /// The per-line object of the batch input file
    pub fn new() -> BatchRequestInput {
        BatchRequestInput {
            custom_id: None,
            method: None,
            url: None,
        }
    }
}
/// The HTTP method to be used for the request. Currently only `POST` is supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "POST")]
    Post,
}

impl Default for Method {
    fn default() -> Method {
        Self::Post
    }
}

