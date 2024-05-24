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
pub struct ModifyAssistantRequest {
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The name of the assistant. The maximum length is 256 characters. 
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// The description of the assistant. The maximum length is 512 characters. 
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters. 
    #[serde(rename = "instructions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Option<String>>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`. 
    #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<models::AssistantObjectToolsInner>>,
    #[serde(rename = "tool_resources", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<Option<Box<models::ModifyAssistantRequestToolResources>>>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. 
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both. 
    #[serde(rename = "top_p", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<Option<f64>>,
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Box<models::AssistantsApiResponseFormatOption>>,
}

impl ModifyAssistantRequest {
    pub fn new() -> ModifyAssistantRequest {
        ModifyAssistantRequest {
            model: None,
            name: None,
            description: None,
            instructions: None,
            tools: None,
            tool_resources: None,
            metadata: None,
            temperature: None,
            top_p: None,
            response_format: None,
        }
    }
}

