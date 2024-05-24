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
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    /// The ID of the message that was created by this run step.
    #[serde(rename = "message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    pub fn new() -> RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
        RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
            message_id: None,
        }
    }
}
