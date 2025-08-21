//! Legacy enums - these are now moved to types.rs in the library
//! Kept for backward compatibility

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum RequestOption {
    Validation = 1,
    Classification = 2,
    ValidationAndClassification = 3,
}
