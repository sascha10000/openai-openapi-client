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

/// AssistantObject : Represents an `assistant` that can call the model and use tools.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantObject {
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `assistant`.
    #[serde(rename = "object")]
    pub object: Object,
    /// The Unix timestamp (in seconds) for when the assistant was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The name of the assistant. The maximum length is 256 characters. 
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 512 characters. 
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them. 
    #[serde(rename = "model")]
    pub model: String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters. 
    #[serde(rename = "instructions", deserialize_with = "Option::deserialize")]
    pub instructions: Option<String>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`. 
    #[serde(rename = "tools")]
    pub tools: Vec<models::AssistantObjectToolsInner>,
    #[serde(rename = "tool_resources", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<Option<Box<models::AssistantObjectToolResources>>>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<serde_json::Value>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. 
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both. 
    #[serde(rename = "top_p", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<Option<f64>>,
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Box<models::AssistantsApiResponseFormatOption>>,
}

impl AssistantObject {
    /// Represents an `assistant` that can call the model and use tools.
    pub fn new(id: String, object: Object, created_at: i32, name: Option<String>, description: Option<String>, model: String, instructions: Option<String>, tools: Vec<models::AssistantObjectToolsInner>, metadata: Option<serde_json::Value>) -> AssistantObject {
        AssistantObject {
            id,
            object,
            created_at,
            name,
            description,
            model,
            instructions,
            tools,
            tool_resources: None,
            metadata,
            temperature: None,
            top_p: None,
            response_format: None,
        }
    }
}
/// The object type, which is always `assistant`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "assistant")]
    Assistant,
}

impl Default for Object {
    fn default() -> Object {
        Self::Assistant
    }
}

