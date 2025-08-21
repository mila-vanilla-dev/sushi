use crate::models::customer::Customer;
use crate::models::order_item::OrderItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub order_id: String,
    pub customer: Customer,
    pub items: Vec<OrderItem>,
    pub special_instructions: Option<String>,
    pub pickup: bool,
}
