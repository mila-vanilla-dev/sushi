//! Type definitions and enums

/// Result of address validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressValidationResult {
    /// Address is valid and can be used for shipping
    Valid,
    /// Address is ambiguous, multiple candidates found
    Ambiguous,
    /// Address is invalid, cannot be used for shipping
    Invalid,
    /// No candidate addresses found
    NoCandidates,
}

/// Rate request options for UPS API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateRequestOptions {
    /// Basic rate calculation
    Rate,
    /// Rate comparison across all UPS services
    Shop,
    /// Rate calculation with transit time information
    RateTimeInTransit,
    /// Rate comparison with transit time for all UPS services
    ShopTimeInTransit,
}

impl RateRequestOptions {
    /// Convert to API string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            RateRequestOptions::Rate => "Rate",
            RateRequestOptions::Shop => "Shop",
            RateRequestOptions::RateTimeInTransit => "RateTimeInTransit",
            RateRequestOptions::ShopTimeInTransit => "ShopTimeInTransit",
        }
    }
}

/// UPS service codes for shipping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpsServiceCode {
    /// UPS Ground
    Ground,
    /// UPS 3 Day Select
    ThreeDaySelect,
    /// UPS 2nd Day Air
    SecondDayAir,
    /// UPS Next Day Air Saver
    NextDayAirSaver,
    /// UPS Next Day Air
    NextDayAir,
    /// UPS Express
    Express,
}

impl UpsServiceCode {
    /// Get the UPS service code as a string
    pub fn code(&self) -> &'static str {
        match self {
            UpsServiceCode::Ground => "03",
            UpsServiceCode::ThreeDaySelect => "12",
            UpsServiceCode::SecondDayAir => "02",
            UpsServiceCode::NextDayAirSaver => "13",
            UpsServiceCode::NextDayAir => "01",
            UpsServiceCode::Express => "07",
        }
    }

    /// Get the human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            UpsServiceCode::Ground => "UPS Ground",
            UpsServiceCode::ThreeDaySelect => "UPS 3 Day Select",
            UpsServiceCode::SecondDayAir => "UPS 2nd Day Air",
            UpsServiceCode::NextDayAirSaver => "UPS Next Day Air Saver",
            UpsServiceCode::NextDayAir => "UPS Next Day Air",
            UpsServiceCode::Express => "UPS Express",
        }
    }
}

/// Package dimensions and weight for UPS shipping calculations
///
/// # UPS Billing Weight Calculation
///
/// UPS determines billing weight using the **greater of**:
/// 1. **Actual weight** (the physical weight you specify)
/// 2. **Dimensional weight** = (Length × Width × Height) ÷ 139
/// 3. **Minimum billing weight** = 4.0 lbs (UPS minimum for most services)
///
/// ## Example:
/// - Package: 10" × 8" × 6", weighs 2.0 lbs
/// - Dimensional weight: (10 × 8 × 6) ÷ 139 = 3.45 lbs
/// - Billing weight: max(2.0, 3.45, 4.0) = **4.0 lbs** (due to minimum)
///
/// This is why you might see a billing weight of 4.0 lbs even for lighter packages.
#[derive(Debug, Clone)]
pub struct PackageDimensions {
    /// Length in inches
    pub length: f32,
    /// Width in inches  
    pub width: f32,
    /// Height in inches
    pub height: f32,
    /// Weight in pounds (actual physical weight)
    ///
    /// Note: UPS will use the greater of this weight, dimensional weight,
    /// or minimum billing weight (typically 4.0 lbs) for rate calculation.
    pub weight: f32,
}

impl Default for PackageDimensions {
    fn default() -> Self {
        PackageDimensions {
            length: 10.0,
            width: 8.0,
            height: 6.0,
            // Note: This 2.0 lbs will result in a 4.0 lbs billing weight due to:
            // - Dimensional weight: (10×8×6)÷139 = 3.45 lbs
            // - UPS minimum billing weight: 4.0 lbs
            // Billing weight = max(2.0, 3.45, 4.0) = 4.0 lbs
            weight: 2.0,
        }
    }
}

/// Parameters for shipping rate requests
#[derive(Debug, Clone)]
pub struct ShippingRateRequest<'a> {
    /// Ship from address
    pub ship_from: &'a crate::models::ups_request::AddressKeyFormat,
    /// Ship to address
    pub ship_to: &'a crate::models::address::Address,
    /// Customer name for shipment
    pub customer_name: &'a str,
    /// Rate request option type
    pub request_option: RateRequestOptions,
    /// UPS service code
    pub service_code: UpsServiceCode,
    /// Package dimensions and weight
    pub dimensions: PackageDimensions,
}
