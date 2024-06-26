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
pub struct ThreadObjectToolResourcesFileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread. 
    #[serde(rename = "vector_store_ids", skip_serializing_if = "Option::is_none")]
    pub vector_store_ids: Option<Vec<String>>,
}

impl ThreadObjectToolResourcesFileSearch {
    pub fn new() -> ThreadObjectToolResourcesFileSearch {
        ThreadObjectToolResourcesFileSearch {
            vector_store_ids: None,
        }
    }
}

