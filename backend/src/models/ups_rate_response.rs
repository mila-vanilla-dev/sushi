use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSRateResponse {
    #[serde(rename = "RateResponse")]
    pub rate_response: RateResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateResponse {
    #[serde(rename = "Response")]
    pub response: RateResponseInfo,
    #[serde(rename = "RatedShipment")]
    pub rated_shipment: Vec<RatedShipment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateResponseInfo {
    #[serde(rename = "ResponseStatus")]
    pub response_status: ResponseStatus,
    #[serde(rename = "Alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Vec<Alert>>,
    #[serde(
        rename = "TransactionReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_reference: Option<TransactionReference>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseStatus {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alert {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionReference {
    #[serde(rename = "CustomerContext")]
    pub customer_context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RatedShipment {
    #[serde(rename = "Service")]
    pub service: Service,
    #[serde(rename = "RatedShipmentAlert", skip_serializing_if = "Option::is_none")]
    pub rated_shipment_alert: Option<Vec<Alert>>,
    #[serde(rename = "BillingWeight", skip_serializing_if = "Option::is_none")]
    pub billing_weight: Option<BillingWeight>,
    #[serde(rename = "TransportationCharges")]
    pub transportation_charges: Charges,
    #[serde(rename = "BaseServiceCharge", skip_serializing_if = "Option::is_none")]
    pub base_service_charge: Option<Charges>,
    #[serde(
        rename = "ServiceOptionsCharges",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_options_charges: Option<Charges>,
    #[serde(rename = "TaxCharges", skip_serializing_if = "Option::is_none")]
    pub tax_charges: Option<Vec<TaxCharge>>,
    #[serde(rename = "TotalCharges")]
    pub total_charges: Charges,
    #[serde(
        rename = "TotalChargesWithTaxes",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_charges_with_taxes: Option<Charges>,
    #[serde(
        rename = "NegotiatedRateCharges",
        skip_serializing_if = "Option::is_none"
    )]
    pub negotiated_rate_charges: Option<NegotiatedRateCharges>,
    #[serde(rename = "GuaranteedDelivery", skip_serializing_if = "Option::is_none")]
    pub guaranteed_delivery: Option<GuaranteedDelivery>,
    #[serde(rename = "RatedPackage")]
    pub rated_package: Vec<RatedPackage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingWeight {
    #[serde(rename = "UnitOfMeasurement")]
    pub unit_of_measurement: UnitOfMeasurement,
    #[serde(rename = "Weight")]
    pub weight: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Charges {
    #[serde(rename = "CurrencyCode")]
    pub currency_code: String,
    #[serde(rename = "MonetaryValue")]
    pub monetary_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaxCharge {
    #[serde(rename = "Type")]
    pub tax_type: String,
    #[serde(rename = "MonetaryValue")]
    pub monetary_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NegotiatedRateCharges {
    #[serde(rename = "TotalCharge", skip_serializing_if = "Option::is_none")]
    pub total_charge: Option<Charges>,
    #[serde(
        rename = "TotalChargesWithTaxes",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_charges_with_taxes: Option<Charges>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuaranteedDelivery {
    #[serde(rename = "BusinessDaysInTransit")]
    pub business_days_in_transit: String,
    #[serde(rename = "DeliveryByTime")]
    pub delivery_by_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RatedPackage {
    #[serde(rename = "TransportationCharges")]
    pub transportation_charges: Charges,
    #[serde(rename = "BaseServiceCharge", skip_serializing_if = "Option::is_none")]
    pub base_service_charge: Option<Charges>,
    #[serde(
        rename = "ServiceOptionsCharges",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_options_charges: Option<Charges>,
    #[serde(rename = "TotalCharges")]
    pub total_charges: Charges,
    #[serde(rename = "Weight")]
    pub weight: String,
    #[serde(rename = "BillingWeight", skip_serializing_if = "Option::is_none")]
    pub billing_weight: Option<BillingWeight>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitOfMeasurement {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}
