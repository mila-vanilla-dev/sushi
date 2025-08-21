use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XAVResponse {
    #[serde(rename = "XAVResponse")]
    pub xav_response: XAVResponseBody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XAVResponseBody {
    #[serde(rename = "Response")]
    pub response: Response,
    #[serde(rename = "ValidAddressIndicator")]
    pub valid_address_indicator: Option<String>,
    #[serde(rename = "AmbiguousAddressIndicator")]
    pub ambiguous_address_indicator: Option<String>,
    #[serde(rename = "NoCandidatesIndicator")]
    pub no_candidates_indicator: Option<String>,
    #[serde(rename = "AddressClassification")]
    pub address_classification: Option<AddressClassification>,
    #[serde(rename = "Candidate")]
    pub candidate: Option<Vec<Candidate>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    #[serde(rename = "ResponseStatus")]
    pub response_status: ResponseStatus,
    #[serde(rename = "Alert")]
    pub alert: Option<Vec<Alert>>,
    #[serde(rename = "TransactionReference")]
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
pub struct AddressClassification {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    #[serde(rename = "AddressClassification")]
    pub address_classification: Option<AddressClassification>,
    #[serde(rename = "AddressKeyFormat")]
    pub address_key_format: Option<AddressKeyFormatCandidate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressKeyFormatCandidate {
    #[serde(rename = "ConsigneeName")]
    pub consignee_name: Option<String>,
    #[serde(rename = "AttentionName")]
    pub attention_name: Option<String>,
    #[serde(rename = "AddressLine")]
    pub address_line: Option<Vec<String>>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "PoliticalDivision2")]
    pub political_division2: Option<String>,
    #[serde(rename = "PoliticalDivision1")]
    pub political_division1: Option<String>,
    #[serde(rename = "PostcodePrimaryLow")]
    pub postcode_primary_low: Option<String>,
    #[serde(rename = "PostcodeExtendedLow")]
    pub postcode_extended_low: Option<String>,
    #[serde(rename = "Urbanization")]
    pub urbanization: Option<String>,
    #[serde(rename = "CountryCode")]
    pub country_code: Option<String>,
}
