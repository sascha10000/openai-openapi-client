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

/// MessageStreamEventOneOf4 : Occurs when a [message](/docs/api-reference/messages/object) ends before it is completed.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageStreamEventOneOf4 {
    #[serde(rename = "event")]
    pub event: Event,
    #[serde(rename = "data")]
    pub data: Box<models::MessageObject>,
}

impl MessageStreamEventOneOf4 {
    /// Occurs when a [message](/docs/api-reference/messages/object) ends before it is completed.
    pub fn new(event: Event, data: models::MessageObject) -> MessageStreamEventOneOf4 {
        MessageStreamEventOneOf4 {
            event,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Event {
    #[serde(rename = "thread.message.incomplete")]
    ThreadPeriodMessagePeriodIncomplete,
}

impl Default for Event {
    fn default() -> Event {
        Self::ThreadPeriodMessagePeriodIncomplete
    }
}

