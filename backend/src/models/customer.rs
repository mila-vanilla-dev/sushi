use serde::{Deserialize, Serialize};

use crate::models::address::Address;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub shipping_address: Address,
    pub billing_address: Address,
}
