use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}
