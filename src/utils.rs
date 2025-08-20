//! Utility functions for order and UPS data handling

use crate::{
    Result as UpsResult, UpsClient,
    models::{
        address::Address, order::Order, ship_from::ShipFrom, ups_api_response::UPSApiResponse,
        ups_rate_response::UPSRateResponse, ups_request::AddressKeyFormat,
    },
    types::{
        AddressValidationResult, PackageDimensions, RateRequestOptions, ShippingRateRequest,
        UpsServiceCode,
    },
};
use std::fs;

/// Load ship-from data from JSON file
pub fn load_ship_from_data(path: &str) -> UpsResult<ShipFrom> {
    let json = fs::read_to_string(path)
        .map_err(|e| crate::error::UpsError::Config(format!("Failed to read {}: {}", path, e)))?;

    serde_json::from_str(&json)
        .map_err(|e| crate::error::UpsError::Parse(format!("Failed to parse {}: {}", path, e)))
}

/// Load order data from JSON file
pub fn load_order_data(path: &str) -> UpsResult<Order> {
    let json = fs::read_to_string(path)
        .map_err(|e| crate::error::UpsError::Config(format!("Failed to read {}: {}", path, e)))?;

    serde_json::from_str(&json)
        .map_err(|e| crate::error::UpsError::Parse(format!("Failed to parse {}: {}", path, e)))
}

/// Display order information
pub fn display_order_info(order: &Order, ship_from: &ShipFrom) {
    tracing::info!("Order ID: {}", order.order_id);
    tracing::info!("Pickup: {}", order.pickup);
    tracing::info!("From Address: {:?}", ship_from.from);

    if let Some(instructions) = &order.special_instructions {
        tracing::info!("Special Instructions: {}", instructions);
    }

    display_customer_info(&order.customer);
    display_order_items(&order.items);
    display_ship_from_address(&ship_from.from);
}

/// Display customer information including addresses
pub fn display_customer_info(customer: &crate::models::customer::Customer) {
    tracing::info!("Customer First Name: {}", customer.first_name);
    tracing::info!("Customer Last Name: {}", customer.last_name);
    tracing::info!("Customer Email: {}", customer.email);
    tracing::info!("Customer Phone: {}", customer.phone);

    tracing::info!("Shipping Address:");
    display_address(&customer.shipping_address, "  ");

    tracing::info!("Billing Address:");
    display_address(&customer.billing_address, "  ");
}

/// Display a generic address with indentation
pub fn display_address(address: &Address, indent: &str) {
    tracing::info!("{}Address: {}", indent, address.address);
    tracing::info!("{}City: {}", indent, address.city);
    tracing::info!("{}State: {}", indent, address.state);
    tracing::info!("{}Postal Code: {}", indent, address.postal_code);
    tracing::info!("{}Country: {}", indent, address.country);
}

/// Display order items
pub fn display_order_items(items: &[crate::models::order_item::OrderItem]) {
    tracing::info!("Items:");
    tracing::info!("---");
    for item in items {
        tracing::info!("  Product ID: {}", item.product_id);
        tracing::info!("  Name: {}", item.name);
        tracing::info!("  Quantity: {}", item.quantity);
        tracing::info!("  Unit Price: ${:.2}", item.unit_price);
        tracing::info!("  Total: ${:.2}", item.unit_price * item.quantity as f64);
        tracing::info!("---");
    }
}

/// Display ship-from address information
pub fn display_ship_from_address(from: &AddressKeyFormat) {
    tracing::info!("Ships from Address:");
    tracing::info!("  Consignee Name: {}", from.consignee_name);
    tracing::info!("  Building Name: {}", from.building_name);
    tracing::info!("  Address Lines: {:?}", from.address_line);
    tracing::info!("  Region: {}", from.region);
    tracing::info!("  Political Division 2: {}", from.political_division2);
    tracing::info!("  Political Division 1: {}", from.political_division1);
    if let Some(postcode) = &from.postcode_primary_low {
        tracing::info!("  Postcode Primary Low: {}", postcode);
    }
    tracing::info!("  Postcode Extended Low: {}", from.postcode_extended_low);
    if let Some(urbanization) = &from.urbanization {
        tracing::info!("  Urbanization: {}", urbanization);
    }
    tracing::info!("  Country Code: {}", from.country_code);
}

/// Display address validation results
pub fn display_validation_results(response: &UPSApiResponse, result: &AddressValidationResult) {
    match response {
        UPSApiResponse::Success(xav_response) => {
            tracing::info!("\n=== UPS Address Validation Response (SUCCESS) ===");
            let response_body = &xav_response.xav_response;
            tracing::info!("Response Status:");
            tracing::info!("  Code: {}", response_body.response.response_status.code);
            tracing::info!(
                "  Description: {}",
                response_body.response.response_status.description
            );

            if let Some(candidates) = &response_body.candidate {
                tracing::info!("Candidate Addresses ({} found):", candidates.len());
                for (i, candidate) in candidates.iter().enumerate() {
                    tracing::info!("  Candidate {}:", i + 1);
                    if let Some(address_format) = &candidate.address_key_format {
                        if let Some(lines) = &address_format.address_line {
                            tracing::info!("    Address Lines: {:?}", lines);
                        }
                        if let Some(city) = &address_format.political_division2 {
                            tracing::info!("    City: {}", city);
                        }
                        if let Some(state) = &address_format.political_division1 {
                            tracing::info!("    State: {}", state);
                        }
                        if let Some(postcode) = &address_format.postcode_primary_low {
                            tracing::info!("    Postal Code: {}", postcode);
                        }
                    }
                    tracing::info!("    ---");
                }
            }
            tracing::info!("=== End XAV Response ===\n");
        }
        UPSApiResponse::Error(error_response) => {
            tracing::info!("\n=== UPS Address Validation Response (ERROR) ===");
            tracing::info!("Errors:");
            for error in &error_response.response.errors {
                tracing::info!("  Code: {}", error.code);
                tracing::info!("  Message: {}", error.message);
                tracing::info!("  ---");
            }
            tracing::info!("=== End Error Response ===\n");
        }
    }

    tracing::info!("Validation Result: {:?}", result);
}

/// Get and display shipping rates
pub async fn get_and_display_rates(
    client: &UpsClient,
    ship_from: &AddressKeyFormat,
    order: &Order,
    access_token: &str,
) -> UpsResult<()> {
    let customer_name = format!("{} {}", order.customer.first_name, order.customer.last_name);
    let dimensions = PackageDimensions::default(); // Using default package dimensions

    let shipping_request = ShippingRateRequest {
        ship_from,
        ship_to: &order.customer.shipping_address,
        customer_name: &customer_name,
        request_option: RateRequestOptions::Rate,
        service_code: UpsServiceCode::Ground,
        dimensions,
    };

    let rate_response = client
        .get_shipping_rates(&shipping_request, access_token)
        .await?;

    display_rate_response(&rate_response);
    Ok(())
}

/// Display rate response
pub fn display_rate_response(rate_response: &UPSRateResponse) {
    tracing::info!("\n=== UPS Rate Response ===");

    let response = &rate_response.rate_response;
    tracing::info!("Response Status:");
    tracing::info!("  Code: {}", response.response.response_status.code);
    tracing::info!(
        "  Description: {}",
        response.response.response_status.description
    );

    if let Some(transaction_ref) = &response.response.transaction_reference {
        tracing::info!("Transaction Reference:");
        tracing::info!("  Customer Context: {}", transaction_ref.customer_context);
    }

    if let Some(alerts) = &response.response.alert {
        tracing::info!("Alerts:");
        for alert in alerts {
            tracing::info!("  Code: {}, Description: {}", alert.code, alert.description);
        }
    }

    tracing::info!(
        "\nRated Shipments ({} found):",
        response.rated_shipment.len()
    );
    for (i, rated_shipment) in response.rated_shipment.iter().enumerate() {
        tracing::info!("  Shipment {}:", i + 1);
        tracing::info!(
            "    Service: {} - {}",
            rated_shipment.service.code,
            rated_shipment
                .service
                .description
                .as_ref()
                .unwrap_or(&"Unknown".to_string())
        );

        if let Some(billing_weight) = &rated_shipment.billing_weight {
            // Note: Billing weight is calculated as max(actual_weight, dimensional_weight, minimum_weight)
            // UPS minimum is typically 4.0 lbs, so lightweight packages will show 4.0 lbs billing weight
            tracing::info!(
                "    Billing Weight: {} {}",
                billing_weight.weight,
                billing_weight.unit_of_measurement.code
            );
        }

        tracing::info!(
            "    Transportation Charges: {} {}",
            rated_shipment.transportation_charges.monetary_value,
            rated_shipment.transportation_charges.currency_code
        );

        if let Some(base_charge) = &rated_shipment.base_service_charge {
            tracing::info!(
                "    Base Service Charge: {} {}",
                base_charge.monetary_value,
                base_charge.currency_code
            );
        }

        tracing::info!(
            "    Total Charges: {} {}",
            rated_shipment.total_charges.monetary_value,
            rated_shipment.total_charges.currency_code
        );

        if let Some(negotiated) = &rated_shipment.negotiated_rate_charges
            && let Some(total) = &negotiated.total_charge
        {
            tracing::info!(
                "    Negotiated Rate: {} {}",
                total.monetary_value,
                total.currency_code
            );
        }

        if let Some(guaranteed) = &rated_shipment.guaranteed_delivery {
            tracing::info!(
                "    Guaranteed Delivery: {} business days by {}",
                guaranteed.business_days_in_transit,
                guaranteed.delivery_by_time
            );
        }

        tracing::info!("    ---");
    }

    tracing::info!("=== End Rate Response ===\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ship_from_data() {
        // This test would require a sample JSON file
        // In a real scenario, you'd test with a valid path
        let result = load_ship_from_data("non_existent_file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_order_data() {
        // This test would require a sample JSON file
        // In a real scenario, you'd test with a valid path
        let result = load_order_data("non_existent_file.json");
        assert!(result.is_err());
    }
}
