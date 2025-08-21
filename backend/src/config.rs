//! Configuration management for UPS API

use std::env;

/// Configuration structure to hold UPS API credentials and settings
#[derive(Debug, Clone)]
pub struct UpsConfig {
    /// UPS API URL (e.g., https://wwwcie.ups.com for testing)
    pub api_url: String,
    /// UPS Client ID for OAuth authentication
    pub client_id: String,
    /// UPS Client Secret for OAuth authentication
    pub client_secret: String,
    /// UPS Merchant ID (same as shipper number)
    pub merchant_id: String,
}

impl UpsConfig {
    /// Create a new UpsConfig from environment variables
    ///
    /// # Environment Variables
    ///
    /// - `UPS_API_URL`: UPS API base URL (optional, defaults to CIE)
    /// - `UPS_CLIENT_ID`: OAuth client ID
    /// - `UPS_CLIENT_SECRET`: OAuth client secret
    /// - `UPS_MERCHANT_ID`: Merchant/Shipper ID
    ///
    /// # Errors
    ///
    /// Returns an error if any required environment variable is missing
    pub fn from_env() -> Result<Self, String> {
        let api_url =
            env::var("UPS_API_URL").unwrap_or_else(|_| "https://wwwcie.ups.com".to_string());
        let client_id = env::var("UPS_CLIENT_ID").map_err(|_| "UPS_CLIENT_ID not set")?;
        let client_secret =
            env::var("UPS_CLIENT_SECRET").map_err(|_| "UPS_CLIENT_SECRET not set")?;
        let merchant_id = env::var("UPS_MERCHANT_ID").map_err(|_| "UPS_MERCHANT_ID not set")?;

        Ok(UpsConfig {
            api_url,
            client_id,
            client_secret,
            merchant_id,
        })
    }

    /// Create a new UpsConfig with explicit values
    pub fn new(
        api_url: String,
        client_id: String,
        client_secret: String,
        merchant_id: String,
    ) -> Self {
        UpsConfig {
            api_url,
            client_id,
            client_secret,
            merchant_id,
        }
    }

    /// Display configuration (masking sensitive data)
    pub fn display(&self) {
        tracing::info!("UPS API URL: {}", self.api_url);
        tracing::info!("UPS Client ID: {}", self.client_id);
        tracing::info!(
            "UPS Client Secret: {}",
            "*".repeat(self.client_secret.len())
        );
        tracing::info!("UPS Merchant ID: {}", "*".repeat(self.merchant_id.len()));
    }
}
