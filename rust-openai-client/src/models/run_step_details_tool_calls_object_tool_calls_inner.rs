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
pub enum RunStepDetailsToolCallsObjectToolCallsInner {
    RunStepDetailsToolCallsCodeObject(Box<models::RunStepDetailsToolCallsCodeObject>),
    RunStepDetailsToolCallsFileSearchObject(Box<models::RunStepDetailsToolCallsFileSearchObject>),
    RunStepDetailsToolCallsFunctionObject(Box<models::RunStepDetailsToolCallsFunctionObject>),
}

impl Default for RunStepDetailsToolCallsObjectToolCallsInner {
    fn default() -> Self {
        Self::RunStepDetailsToolCallsCodeObject(Default::default())
    }
}
/// The type of tool call. This is always going to be `code_interpreter` for this type of tool call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "file_search")]
    FileSearch,
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::CodeInterpreter
    }
}

