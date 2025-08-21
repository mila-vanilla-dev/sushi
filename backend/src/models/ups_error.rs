use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSErrorResponse {
    pub response: UPSErrorResponseBody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSErrorResponseBody {
    pub errors: Vec<UPSError>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSError {
    pub code: String,
    pub message: String,
}
