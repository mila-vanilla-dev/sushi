//! UPS API Client Library
//!
//! This library provides a convenient interface for interacting with UPS APIs,
//! including address validation and shipping rate calculations.

pub mod auth;
pub mod client;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod middleware;
pub mod models;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use client::UpsClient;
pub use config::UpsConfig;
pub use error::{Result, UpsError};
pub use types::{AddressValidationResult, RateRequestOptions, ShippingRateRequest};
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::postgres::PgPool;

/// Application state that holds the UPS client and access token
#[derive(Debug, Clone)]
pub struct AppState {
    pub ups_client: UpsClient,
    pub access_token: String,
    pub user_store: Arc<RwLock<endpoints::auth::UserStore>>,
    pub db_pool: PgPool,
}

pub use models::{
    address::Address, customer::Customer, order::Order, order_item::OrderItem,
    ups_api_response::UPSApiResponse, ups_rate_request::UPSRateRequest,
    ups_rate_response::UPSRateResponse,
};
