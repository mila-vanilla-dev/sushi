use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub product_id: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price: f64,
}
