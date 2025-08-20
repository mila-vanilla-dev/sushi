//! UPS API Client implementation

use crate::{
    config::UpsConfig,
    error::{Result, UpsError},
    models::{
        address::Address,
        ups_api_response::UPSApiResponse,
        ups_rate_request::*,
        ups_rate_response::UPSRateResponse,
        ups_request::{AddressKeyFormat, UPSAddressValidationRequest, XAVRequest},
    },
    types::{AddressValidationResult, PackageDimensions, ShippingRateRequest, UpsServiceCode},
};
use base64::{Engine as _, engine::general_purpose};

/// Main UPS API client
#[derive(Debug, Clone)]
pub struct UpsClient {
    config: UpsConfig,
    client: reqwest::Client,
    debug: bool,
}

impl UpsClient {
    /// Create a new UPS client
    pub fn new(config: UpsConfig) -> Self {
        UpsClient {
            config,
            client: reqwest::Client::new(),
            debug: false,
        }
    }

    /// Enable or disable debug logging
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Get OAuth access token from UPS API
    pub async fn get_access_token(&self) -> Result<String> {
        if self.debug {
            tracing::info!("\n=== Getting OAuth Token ===");
        }

        let oauth_url = format!("{}/security/v1/oauth/token", self.config.api_url);
        let auth_string = format!("{}:{}", self.config.client_id, self.config.client_secret);
        let auth_header = format!("Basic {}", general_purpose::STANDARD.encode(auth_string));
        let oauth_params = [("grant_type", "client_credentials")];

        if self.debug {
            tracing::info!("=== DEBUG: OAuth Request ===");
            tracing::info!("URL: {}", oauth_url);
            tracing::info!("=== END DEBUG: OAuth Request ===\n");
        }

        let response = self
            .client
            .post(&oauth_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_header)
            .header("x-merchant-id", &self.config.merchant_id)
            .form(&oauth_params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            if self.debug {
                tracing::info!("=== DEBUG: OAuth Error Response ===");
                tracing::info!("{}", error_text);
                tracing::info!("=== END DEBUG: OAuth Error Response ===\n");
            }
            return Err(UpsError::Auth(format!("OAuth failed: {}", error_text)));
        }

        let oauth_text = response.text().await?;

        if self.debug {
            tracing::info!("=== DEBUG: OAuth Raw Response ===");
            tracing::info!("{}", oauth_text);
            tracing::info!("=== END DEBUG: OAuth Raw Response ===\n");
        }

        let oauth_json: serde_json::Value = serde_json::from_str(&oauth_text)?;
        let access_token = oauth_json["access_token"]
            .as_str()
            .ok_or_else(|| UpsError::Parse("No access token in response".to_string()))?;

        if self.debug {
            tracing::info!("OAuth Token obtained successfully");
            tracing::info!(
                "Token type: {}",
                oauth_json["token_type"].as_str().unwrap_or("unknown")
            );
            tracing::info!(
                "Expires in: {} seconds",
                oauth_json["expires_in"].as_u64().unwrap_or(0)
            );
        }

        Ok(access_token.to_string())
    }

    /// Validate an address using UPS Address Validation API
    pub async fn validate_address(
        &self,
        address: &AddressKeyFormat,
        access_token: &str,
    ) -> Result<(UPSApiResponse, AddressValidationResult)> {
        if self.debug {
            tracing::info!("\n=== Validating Address ===");
        }

        let validation_url = format!("{}/api/addressvalidation/v2/1", self.config.api_url);

        let body = UPSAddressValidationRequest {
            xav_request: XAVRequest {
                address_key_format: address.clone(),
            },
        };

        if self.debug {
            tracing::info!("=== DEBUG: Address Validation Request ===");
            tracing::info!("URL: {}", validation_url);
            tracing::info!("Request Body:");
            tracing::info!("{}", serde_json::to_string_pretty(&body)?);
            tracing::info!("=== END DEBUG: Address Validation Request ===\n");
        }

        let response = self
            .client
            .post(&validation_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("transId", "address-validation-request")
            .header("transactionSrc", "ups-api-client")
            .json(&body)
            .send()
            .await?;

        let response_text = response.text().await?;

        if self.debug {
            tracing::info!("=== DEBUG: Raw Response ===");
            tracing::info!("{}", response_text);
            tracing::info!("=== END DEBUG: Raw Response ===\n");
        }

        let api_response: UPSApiResponse = serde_json::from_str(&response_text)?;
        let validation_result = self.determine_validation_result(&api_response);

        Ok((api_response, validation_result))
    }

    /// Get shipping rates for a shipment
    pub async fn get_shipping_rates(
        &self,
        request: &ShippingRateRequest<'_>,
        access_token: &str,
    ) -> Result<UPSRateResponse> {
        if self.debug {
            tracing::info!("\n=== Getting Shipping Rate ===");
        }

        let rate_url = format!(
            "{}/api/rating/v2409/{}",
            self.config.api_url,
            request.request_option.as_str()
        );

        let rate_request = self.create_rate_request(
            request.ship_from,
            request.ship_to,
            request.customer_name,
            request.service_code,
            request.dimensions.clone(),
        )?;

        if self.debug {
            tracing::info!("=== DEBUG: Rate Request ===");
            tracing::info!("URL: {}", rate_url);
            tracing::info!("Request Body:");
            tracing::info!("{}", serde_json::to_string_pretty(&rate_request)?);
            tracing::info!("=== END DEBUG: Rate Request ===\n");
        }

        let response = self
            .client
            .post(&rate_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("transId", "rate-request")
            .header("transactionSrc", "ups-api-client")
            .json(&rate_request)
            .send()
            .await?;

        let response_text = response.text().await?;

        if self.debug {
            tracing::info!("=== DEBUG: Rate Raw Response ===");
            tracing::info!("{}", response_text);
            tracing::info!("=== END DEBUG: Rate Raw Response ===\n");
        }

        // Check if response contains error
        if response_text.contains("\"errors\"") {
            return Err(UpsError::Api(format!("Rate API error: {}", response_text)));
        }

        let rate_response: UPSRateResponse = serde_json::from_str(&response_text)?;
        Ok(rate_response)
    }

    /// Create a rate request from address and shipment details
    fn create_rate_request(
        &self,
        ship_from: &AddressKeyFormat,
        ship_to: &Address,
        customer_name: &str,
        service_code: UpsServiceCode,
        dimensions: PackageDimensions,
    ) -> Result<UPSRateRequest> {
        // Convert AddressKeyFormat to RateAddress for ship from
        let ship_from_address = RateAddress {
            address_line: ship_from.address_line.clone(),
            city: ship_from.political_division2.clone(),
            state_province_code: ship_from.political_division1.clone(),
            postal_code: ship_from.postcode_primary_low.clone().unwrap_or_default(),
            country_code: ship_from.country_code.clone(),
        };

        // Convert Address to RateAddress for ship to
        let ship_to_address = RateAddress {
            address_line: vec![ship_to.address.clone()],
            city: ship_to.city.clone(),
            state_province_code: ship_to.state.clone(),
            postal_code: ship_to.postal_code.clone(),
            country_code: ship_to.country.clone(),
        };

        Ok(UPSRateRequest {
            rate_request: RateRequest {
                request: RateRequestInfo {
                    transaction_reference: TransactionReference {
                        customer_context: "ups-api-client-rate-request".to_string(),
                    },
                },
                shipment: Shipment {
                    shipper: Shipper {
                        name: ship_from.consignee_name.clone(),
                        shipper_number: self.config.merchant_id.clone(),
                        address: ship_from_address.clone(),
                    },
                    ship_to: ShipTo {
                        name: customer_name.to_string(),
                        address: ship_to_address,
                    },
                    ship_from: crate::models::ups_rate_request::ShipFrom {
                        name: ship_from.building_name.clone(),
                        address: ship_from_address,
                    },
                    payment_details: PaymentDetails {
                        shipment_charge: vec![ShipmentCharge {
                            charge_type: "01".to_string(), // Bill Shipper
                            bill_shipper: BillShipper {
                                account_number: self.config.merchant_id.clone(),
                            },
                        }],
                    },
                    service: Service {
                        code: service_code.code().to_string(),
                        description: service_code.description().to_string(),
                    },
                    num_of_pieces: "1".to_string(),
                    package: Package {
                        simple_rate: None,
                        packaging_type: PackagingType {
                            code: "02".to_string(), // Customer Supplied Package
                            description: "Customer Supplied Package".to_string(),
                        },
                        dimensions: Dimensions {
                            unit_of_measurement: UnitOfMeasurement {
                                code: "IN".to_string(),
                                description: "Inches".to_string(),
                            },
                            length: dimensions.length.to_string(),
                            width: dimensions.width.to_string(),
                            height: dimensions.height.to_string(),
                        },
                        package_weight: PackageWeight {
                            unit_of_measurement: UnitOfMeasurement {
                                code: "LBS".to_string(),
                                description: "Pounds".to_string(),
                            },
                            // Note: UPS will calculate billing weight as the greater of:
                            // 1. This actual weight
                            // 2. Dimensional weight: (L×W×H)÷139
                            // 3. Minimum billing weight (typically 4.0 lbs)
                            weight: dimensions.weight.to_string(),
                        },
                    },
                },
            },
        })
    }

    /// Determine validation result from API response
    fn determine_validation_result(
        &self,
        api_response: &UPSApiResponse,
    ) -> AddressValidationResult {
        match api_response {
            UPSApiResponse::Success(xav_response) => {
                let response_body = &xav_response.xav_response;
                if response_body.valid_address_indicator.is_some() {
                    AddressValidationResult::Valid
                } else if response_body.ambiguous_address_indicator.is_some() {
                    AddressValidationResult::Ambiguous
                } else if response_body.no_candidates_indicator.is_some() {
                    AddressValidationResult::NoCandidates
                } else {
                    AddressValidationResult::Invalid
                }
            }
            UPSApiResponse::Error(_) => AddressValidationResult::Invalid,
        }
    }
}
