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

/// RunStepStreamEventOneOf6 : Occurs when a [run step](/docs/api-reference/runs/step-object) expires.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepStreamEventOneOf6 {
    #[serde(rename = "event")]
    pub event: Event,
    #[serde(rename = "data")]
    pub data: Box<models::RunStepObject>,
}

impl RunStepStreamEventOneOf6 {
    /// Occurs when a [run step](/docs/api-reference/runs/step-object) expires.
    pub fn new(event: Event, data: models::RunStepObject) -> RunStepStreamEventOneOf6 {
        RunStepStreamEventOneOf6 {
            event,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "thread.run.step.expired")]
    ThreadPeriodRunPeriodStepPeriodExpired,
}

impl Default for Event {
    fn default() -> Event {
        Self::ThreadPeriodRunPeriodStepPeriodExpired
    }
}
