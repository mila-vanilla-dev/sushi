use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSRateRequest {
    #[serde(rename = "RateRequest")]
    pub rate_request: RateRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateRequest {
    #[serde(rename = "Request")]
    pub request: RateRequestInfo,
    #[serde(rename = "Shipment")]
    pub shipment: Shipment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateRequestInfo {
    #[serde(rename = "TransactionReference")]
    pub transaction_reference: TransactionReference,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionReference {
    #[serde(rename = "CustomerContext")]
    pub customer_context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shipment {
    #[serde(rename = "Shipper")]
    pub shipper: Shipper,
    #[serde(rename = "ShipTo")]
    pub ship_to: ShipTo,
    #[serde(rename = "ShipFrom")]
    pub ship_from: ShipFrom,
    #[serde(rename = "PaymentDetails")]
    pub payment_details: PaymentDetails,
    #[serde(rename = "Service")]
    pub service: Service,
    #[serde(rename = "NumOfPieces")]
    pub num_of_pieces: String,
    #[serde(rename = "Package")]
    pub package: Package,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shipper {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShipperNumber")]
    pub shipper_number: String,
    #[serde(rename = "Address")]
    pub address: RateAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipTo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Address")]
    pub address: RateAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipFrom {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Address")]
    pub address: RateAddress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateAddress {
    #[serde(rename = "AddressLine")]
    pub address_line: Vec<String>,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "StateProvinceCode")]
    pub state_province_code: String,
    #[serde(rename = "PostalCode")]
    pub postal_code: String,
    #[serde(rename = "CountryCode")]
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentDetails {
    #[serde(rename = "ShipmentCharge")]
    pub shipment_charge: Vec<ShipmentCharge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShipmentCharge {
    #[serde(rename = "Type")]
    pub charge_type: String,
    #[serde(rename = "BillShipper")]
    pub bill_shipper: BillShipper,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillShipper {
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    #[serde(rename = "SimpleRate", skip_serializing_if = "Option::is_none")]
    pub simple_rate: Option<SimpleRate>,
    #[serde(rename = "PackagingType")]
    pub packaging_type: PackagingType,
    #[serde(rename = "Dimensions")]
    pub dimensions: Dimensions,
    #[serde(rename = "PackageWeight")]
    pub package_weight: PackageWeight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleRate {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Code")]
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackagingType {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    #[serde(rename = "UnitOfMeasurement")]
    pub unit_of_measurement: UnitOfMeasurement,
    #[serde(rename = "Length")]
    pub length: String,
    #[serde(rename = "Width")]
    pub width: String,
    #[serde(rename = "Height")]
    pub height: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageWeight {
    #[serde(rename = "UnitOfMeasurement")]
    pub unit_of_measurement: UnitOfMeasurement,
    #[serde(rename = "Weight")]
    pub weight: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitOfMeasurement {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}
