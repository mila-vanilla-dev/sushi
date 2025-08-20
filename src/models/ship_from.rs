use crate::models::ups_request::AddressKeyFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipFrom {
    pub from: AddressKeyFormat,
}
