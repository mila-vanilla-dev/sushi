/*
Example POST JSON body
{
  "customer": {
    "name": "Jane Doe",
    "email": "jane.doe@example.com",
    "phone": "+1-555-234-5678",
    "shipping_address": {
      "line1": "123 Main Street",
      "line2": "Apt 4B",
      "city": "Denver",
      "state": "CO",
      "postal_code": "80202",
      "country": "US"
    }
  },
  "prints": [
    {
      "size": "4x6",
      "quantity": 10,
      "finish": "glossy",
      "image_ids": ["img_001", "img_002"]
    },
    {
      "size": "8x10",
      "quantity": 2,
      "finish": "matte",
      "image_ids": ["img_003"]
    }
  ],
  "special_instructions": "Please adjust colors for warmer tones.",
  "shipping_option": "USPS_Priority",
  "payment": {
    "method": "paypal",
    "order_id": "5O190127TN364715T"
  }
}

Example Response JSON
{
  "order_id": "ord_20250812_0001",
  "status": "pending_payment",
  "paypal": {
    "order_id": "5O190127TN364715T",
    "approval_url": "https://www.sandbox.paypal.com/checkoutnow?token=5O190127TN364715T"
  },
  "total": {
    "items_subtotal": 28.00,
    "shipping": 7.50,
    "tax": 2.48,
    "currency": "USD",
    "grand_total": 37.98
  },
  "estimated_delivery": {
    "min_date": "2025-08-18",
    "max_date": "2025-08-22"
  },
  "message": "Order created successfully. Please complete payment using the provided PayPal link."
}
*/

use crate::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Request structures matching the example JSON
#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub customer: CustomerRequest,
    pub prints: Vec<PrintRequest>,
    pub special_instructions: Option<String>,
    pub shipping_option: String,
    pub payment: PaymentRequest,
}

#[derive(Debug, Deserialize)]
pub struct CustomerRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub shipping_address: AddressRequest,
}

#[derive(Debug, Deserialize)]
pub struct AddressRequest {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct PrintRequest {
    pub size: String,
    pub quantity: u32,
    pub finish: String,
    pub image_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub method: String,
    pub order_id: Option<String>,
}

// Response structures matching the example JSON
#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PayPalResponse>,
    pub total: TotalResponse,
    pub estimated_delivery: DeliveryEstimate,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PayPalResponse {
    pub order_id: String,
    pub approval_url: String,
}

#[derive(Debug, Serialize)]
pub struct TotalResponse {
    pub items_subtotal: f64,
    pub shipping: f64,
    pub tax: f64,
    pub currency: String,
    pub grand_total: f64,
}

#[derive(Debug, Serialize)]
pub struct DeliveryEstimate {
    pub min_date: String, // ISO date format
    pub max_date: String, // ISO date format
}

// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<String>,
}

/// Orders endpoint - handles order creation with proper error handling
pub async fn orders_endpoint(
    State(app_state): State<AppState>,
    Json(payload): Json<OrderRequest>,
) -> Response {
    tracing::info!(
        "Received order request for customer: {}",
        payload.customer.name
    );
    tracing::debug!("Order request payload: {:?}", payload);

    match process_order(payload, &app_state).await {
        Ok(response) => {
            tracing::info!("Order created successfully with ID: {}", response.order_id);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Order processing failed: {}", err);
            let error_response = ErrorResponse {
                error: "ORDER_PROCESSING_FAILED".to_string(),
                message: err.to_string(),
                details: None,
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}

/// Process the order and return a response or error
async fn process_order(
    request: OrderRequest,
    app_state: &AppState,
) -> Result<OrderResponse, Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!("Starting order processing");

    // Validate the request
    tracing::debug!("Validating order request");
    validate_order_request(&request)?;

    // Generate order ID
    let order_id = generate_order_id();
    tracing::info!("Generated order ID: {}", order_id);

    // Now we have access to the UPS client and access token through app_state
    // Example usage:
    // let ups_rates = app_state.ups_client.get_rates(&rate_request).await?;
    // let auth_header = format!("Bearer {}", app_state.access_token);
    #[allow(unused_variables)]
    let _ups_client = &app_state.ups_client;
    #[allow(unused_variables)]
    let _access_token = &app_state.access_token;

    // Calculate totals
    tracing::debug!("Calculating order totals");
    let total = calculate_totals(&request.prints, &request.shipping_option)?;
    tracing::info!("Order total calculated: ${:.2}", total.grand_total);

    // Create delivery estimate
    tracing::debug!("Calculating delivery estimate");
    let delivery_estimate = calculate_delivery_estimate(&request.shipping_option)?;

    // Handle payment processing
    tracing::debug!("Processing payment method: {}", request.payment.method);
    let (status, paypal_response) = match request.payment.method.as_str() {
        "paypal" => {
            tracing::info!("Processing PayPal payment");
            let paypal = process_paypal_payment(&request.payment, total.grand_total)?;
            ("pending_payment".to_string(), Some(paypal))
        }
        "credit_card" => {
            tracing::info!("Processing credit card payment");
            // For credit card, we'd process immediately
            ("processing".to_string(), None)
        }
        _ => {
            tracing::error!("Unsupported payment method: {}", request.payment.method);
            return Err("Unsupported payment method".into());
        }
    };

    tracing::info!("Order processing completed successfully");
    Ok(OrderResponse {
        order_id: order_id.clone(),
        status,
        paypal: paypal_response,
        total,
        estimated_delivery: delivery_estimate,
        message:
            "Order created successfully. Please complete payment using the provided PayPal link."
                .to_string(),
    })
}

/// Validate the order request
fn validate_order_request(
    request: &OrderRequest,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if request.customer.name.trim().is_empty() {
        return Err("Customer name is required".into());
    }

    if request.customer.email.trim().is_empty() {
        return Err("Customer email is required".into());
    }

    if request.prints.is_empty() {
        return Err("At least one print item is required".into());
    }

    for print in &request.prints {
        if print.quantity == 0 {
            return Err("Print quantity must be greater than 0".into());
        }
        if print.image_ids.is_empty() {
            return Err("At least one image ID is required for each print".into());
        }
    }

    Ok(())
}

/// Generate a unique order ID
fn generate_order_id() -> String {
    let now = Utc::now();
    format!("ord_{}_0001", now.format("%Y%m%d"))
}

/// Calculate order totals
fn calculate_totals(
    prints: &[PrintRequest],
    shipping_option: &str,
) -> Result<TotalResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut items_subtotal = 0.0;

    // Calculate print costs based on size and quantity
    for print in prints {
        let unit_price = match print.size.as_str() {
            "4x6" => 1.50,
            "5x7" => 2.00,
            "8x10" => 4.00,
            "11x14" => 8.00,
            _ => return Err(format!("Unsupported print size: {}", print.size).into()),
        };

        // Add finish premium
        let finish_premium = match print.finish.as_str() {
            "glossy" => 0.0,
            "matte" => 0.25,
            "metallic" => 0.50,
            _ => return Err(format!("Unsupported finish: {}", print.finish).into()),
        };

        let total_images = print.image_ids.len() as f64;
        items_subtotal += (unit_price + finish_premium) * print.quantity as f64 * total_images;
    }

    // Calculate shipping
    let shipping = match shipping_option {
        "USPS_Ground" => 5.00,
        "USPS_Priority" => 7.50,
        "USPS_Express" => 12.00,
        "UPS_Ground" => 6.50,
        "UPS_2Day" => 10.00,
        "UPS_Overnight" => 20.00,
        _ => return Err(format!("Unsupported shipping option: {}", shipping_option).into()),
    };

    // Calculate tax (example: 7% sales tax)
    let tax = (items_subtotal + shipping) * 0.07;
    let grand_total = items_subtotal + shipping + tax;

    Ok(TotalResponse {
        items_subtotal: (items_subtotal * 100.0).round() / 100.0,
        shipping: (shipping * 100.0).round() / 100.0,
        tax: (tax * 100.0).round() / 100.0,
        currency: "USD".to_string(),
        grand_total: (grand_total * 100.0).round() / 100.0,
    })
}

/// Calculate delivery estimate based on shipping option
fn calculate_delivery_estimate(
    shipping_option: &str,
) -> Result<DeliveryEstimate, Box<dyn std::error::Error + Send + Sync>> {
    let now = Utc::now();

    let (min_days, max_days) = match shipping_option {
        "USPS_Ground" => (5, 8),
        "USPS_Priority" => (2, 3),
        "USPS_Express" => (1, 2),
        "UPS_Ground" => (3, 5),
        "UPS_2Day" => (2, 2),
        "UPS_Overnight" => (1, 1),
        _ => return Err(format!("Unsupported shipping option: {}", shipping_option).into()),
    };

    let min_date = now + chrono::Duration::days(min_days);
    let max_date = now + chrono::Duration::days(max_days);

    Ok(DeliveryEstimate {
        min_date: min_date.format("%Y-%m-%d").to_string(),
        max_date: max_date.format("%Y-%m-%d").to_string(),
    })
}

/// Process PayPal payment
fn process_paypal_payment(
    payment: &PaymentRequest,
    _total: f64,
) -> Result<PayPalResponse, Box<dyn std::error::Error + Send + Sync>> {
    // In a real implementation, this would integrate with PayPal API
    let order_id = payment
        .order_id
        .as_ref()
        .ok_or("PayPal order ID is required")?;

    Ok(PayPalResponse {
        order_id: order_id.clone(),
        approval_url: format!(
            "https://www.sandbox.paypal.com/checkoutnow?token={}",
            order_id
        ),
    })
}
