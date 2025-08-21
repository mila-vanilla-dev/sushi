use crate::models::ups_error::UPSErrorResponse;
use crate::models::ups_response::XAVResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UPSApiResponse {
    Success(Box<XAVResponse>),
    Error(UPSErrorResponse),
}

#[allow(dead_code)]
impl UPSApiResponse {
    pub fn is_success(&self) -> bool {
        matches!(self, UPSApiResponse::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, UPSApiResponse::Error(_))
    }
}
