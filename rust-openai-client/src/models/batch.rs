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
pub struct Batch {
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `batch`.
    #[serde(rename = "object")]
    pub object: Object,
    /// The OpenAI API endpoint used by the batch.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<models::BatchErrors>>,
    /// The ID of the input file for the batch.
    #[serde(rename = "input_file_id")]
    pub input_file_id: String,
    /// The time frame within which the batch should be processed.
    #[serde(rename = "completion_window")]
    pub completion_window: String,
    /// The current status of the batch.
    #[serde(rename = "status")]
    pub status: Status,
    /// The ID of the file containing the outputs of successfully executed requests.
    #[serde(rename = "output_file_id", skip_serializing_if = "Option::is_none")]
    pub output_file_id: Option<String>,
    /// The ID of the file containing the outputs of requests with errors.
    #[serde(rename = "error_file_id", skip_serializing_if = "Option::is_none")]
    pub error_file_id: Option<String>,
    /// The Unix timestamp (in seconds) for when the batch was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The Unix timestamp (in seconds) for when the batch started processing.
    #[serde(rename = "in_progress_at", skip_serializing_if = "Option::is_none")]
    pub in_progress_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch will expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    #[serde(rename = "finalizing_at", skip_serializing_if = "Option::is_none")]
    pub finalizing_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch was completed.
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch failed.
    #[serde(rename = "failed_at", skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch expired.
    #[serde(rename = "expired_at", skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    #[serde(rename = "cancelling_at", skip_serializing_if = "Option::is_none")]
    pub cancelling_at: Option<i32>,
    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    #[serde(rename = "cancelled_at", skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i32>,
    #[serde(rename = "request_counts", skip_serializing_if = "Option::is_none")]
    pub request_counts: Option<Box<models::BatchRequestCounts>>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maxium of 512 characters long. 
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
}

impl Batch {
    pub fn new(id: String, object: Object, endpoint: String, input_file_id: String, completion_window: String, status: Status, created_at: i32) -> Batch {
        Batch {
            id,
            object,
            endpoint,
            errors: None,
            input_file_id,
            completion_window,
            status,
            output_file_id: None,
            error_file_id: None,
            created_at,
            in_progress_at: None,
            expires_at: None,
            finalizing_at: None,
            completed_at: None,
            failed_at: None,
            expired_at: None,
            cancelling_at: None,
            cancelled_at: None,
            request_counts: None,
            metadata: None,
        }
    }
}
/// The object type, which is always `batch`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "batch")]
    Batch,
}

impl Default for Object {
    fn default() -> Object {
        Self::Batch
    }
}
/// The current status of the batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "validating")]
    Validating,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finalizing")]
    Finalizing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Validating
    }
}

