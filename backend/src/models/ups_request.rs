use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UPSAddressValidationRequest {
    #[serde(rename = "XAVRequest")]
    pub xav_request: XAVRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XAVRequest {
    #[serde(rename = "AddressKeyFormat")]
    pub address_key_format: AddressKeyFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressKeyFormat {
    #[serde(rename = "ConsigneeName")]
    pub consignee_name: String,
    #[serde(rename = "BuildingName")]
    pub building_name: String,
    #[serde(rename = "AddressLine")]
    pub address_line: Vec<String>,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "PoliticalDivision2")]
    pub political_division2: String,
    #[serde(rename = "PoliticalDivision1")]
    pub political_division1: String,
    #[serde(rename = "PostcodePrimaryLow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode_primary_low: Option<String>,
    #[serde(rename = "PostcodeExtendedLow")]
    pub postcode_extended_low: String,
    #[serde(rename = "Urbanization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urbanization: Option<String>,
    #[serde(rename = "CountryCode")]
    pub country_code: String,
}
