//! JWT-based authentication utilities

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub email: String, // User email
    pub name: String,  // User name
    pub admin: bool,   // Is admin flag
    pub exp: usize,    // Expiration time (as UTC timestamp)
    pub iat: usize,    // Issued at (as UTC timestamp)
}

/// JWT token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_in: usize,
    pub token_type: String,
}

/// Get JWT secret from environment or use default for development
pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!(
            "JWT_SECRET not found in environment, using default (not secure for production!)"
        );
        "your-secret-key-change-this-in-production".to_string()
    })
}

/// Generate a JWT token for a user
pub fn generate_token(
    user_id: Uuid,
    email: &str,
    name: &str,
    is_admin: bool,
    expires_in_hours: Option<usize>,
) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret();
    let expires_in = expires_in_hours.unwrap_or(24); // Default to 24 hours

    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(expires_in as i64)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        name: name.to_string(),
        admin: is_admin,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(TokenResponse {
        token,
        expires_in: expires_in * 3600, // Convert hours to seconds
        token_type: "Bearer".to_string(),
    })
}

/// Validate and decode a JWT token
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret();
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}

/// Extract token from Authorization header
pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    auth_header.strip_prefix("Bearer ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let name = "Test User";
        let is_admin = false;

        // Generate token
        let token_response = generate_token(user_id, email, name, is_admin, Some(1))
            .expect("Failed to generate token");

        assert_eq!(token_response.token_type, "Bearer");
        assert_eq!(token_response.expires_in, 3600); // 1 hour in seconds

        // Validate token
        let claims = validate_token(&token_response.token).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.name, name);
        assert_eq!(claims.admin, is_admin);
    }

    #[test]
    fn test_token_extraction() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...";
        let auth_header = format!("Bearer {}", token);

        let extracted = extract_token_from_header(&auth_header);
        assert_eq!(extracted, Some(token));

        // Test invalid header
        let invalid_header = "InvalidHeader";
        let extracted = extract_token_from_header(invalid_header);
        assert_eq!(extracted, None);
    }
}
