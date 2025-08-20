use argon2::{
    Argon2,
    password_hash::{
        Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)] // Never serialize password hash
    pub password_hash: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub is_admin: bool,
}

impl User {
    pub fn new(
        email: String,
        name: String,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Validate email format
        validate_email(&email)?;

        // Validate password strength
        validate_password_strength(password)?;

        let password_hash =
            hash_password(password).map_err(|e| format!("Failed to hash password: {}", e))?;

        Ok(User {
            id: uuid::Uuid::new_v4(),
            email,
            name,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_admin: false,
        })
    }

    pub fn update(&mut self, email: Option<String>, name: Option<String>) {
        if let Some(email) = email {
            self.email = email;
        }
        if let Some(name) = name {
            self.name = name;
        }
        self.updated_at = Utc::now();
    }

    pub fn update_password(
        &mut self,
        new_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Validate password strength
        validate_password_strength(new_password)?;

        self.password_hash =
            hash_password(new_password).map_err(|e| format!("Failed to hash password: {}", e))?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, PasswordHashError> {
        verify_password(&self.password_hash, password)
    }

    pub fn set_admin(&mut self, is_admin: bool) {
        self.is_admin = is_admin;
        self.updated_at = Utc::now();
    }

    /// Create a sanitized version of the user for API responses (without sensitive data)
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id,
            email: self.email.clone(),
            name: self.name.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            is_admin: self.is_admin,
        }
    }
}

/// Public user representation for API responses (excludes password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub is_admin: bool,
}

/// Hash a password using Argon2id with secure defaults
fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    use rand::rngs::OsRng;
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id with default params (recommended for password hashing)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verify a password against a stored hash
fn verify_password(hash: &str, password: &str) -> Result<bool, PasswordHashError> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    if password.len() > 128 {
        return Err("Password must be no more than 128 characters long".to_string());
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if !has_uppercase {
        return Err("Password must contain at least one uppercase letter".to_string());
    }

    if !has_lowercase {
        return Err("Password must contain at least one lowercase letter".to_string());
    }

    if !has_digit {
        return Err("Password must contain at least one digit".to_string());
    }

    if !has_special {
        return Err("Password must contain at least one special character".to_string());
    }

    Ok(())
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    if !email.contains('@') {
        return Err("Email must contain @ symbol".to_string());
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Email must have exactly one @ symbol".to_string());
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err("Email must have non-empty local and domain parts".to_string());
    }

    if !parts[1].contains('.') {
        return Err("Email domain must contain at least one dot".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "SecurePass123!";
        let hash = hash_password(password).expect("Failed to hash password");

        // Verify the correct password
        assert!(verify_password(&hash, password).expect("Failed to verify password"));

        // Verify wrong password fails
        assert!(!verify_password(&hash, "WrongPass456!").expect("Failed to verify wrong password"));
    }

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "SecurePass123!",
        )
        .expect("Failed to create user");

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert!(!user.is_admin);
        assert!(!user.password_hash.is_empty());
        assert!(
            user.verify_password("SecurePass123!")
                .expect("Failed to verify password")
        );
    }

    #[test]
    fn test_password_update() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "Old_Password123!",
        )
        .expect("Failed to create user");

        // Update password
        user.update_password("New_Secure_Password456!")
            .expect("Failed to update password");

        // Old password should not work
        assert!(
            !user
                .verify_password("Old_Password123!")
                .expect("Failed to verify old password")
        );

        // New password should work
        assert!(
            user.verify_password("New_Secure_Password456!")
                .expect("Failed to verify new password")
        );
    }

    #[test]
    fn test_password_strength_validation() {
        // Valid password
        assert!(validate_password_strength("StrongPass123!").is_ok());

        // Too short
        assert!(validate_password_strength("Short1!").is_err());

        // No uppercase
        assert!(validate_password_strength("nouppercasepass123!").is_err());

        // No lowercase
        assert!(validate_password_strength("NOLOWERCASEPASS123!").is_err());

        // No digits
        assert!(validate_password_strength("NoDigitsPass!").is_err());

        // No special characters
        assert!(validate_password_strength("NoSpecialChars123").is_err());

        // Too long
        let long_password = "A".repeat(129) + "1!";
        assert!(validate_password_strength(&long_password).is_err());
    }

    #[test]
    fn test_email_validation() {
        // Valid emails
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name+tag@domain.co.uk").is_ok());

        // Invalid emails
        assert!(validate_email("").is_err());
        assert!(validate_email("notanemail").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@@domain.com").is_err());
        assert!(validate_email("user@domain").is_err());
    }

    #[test]
    fn test_public_user_conversion() {
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            "StrongPass123!",
        )
        .expect("Failed to create user");

        let public_user = user.to_public();

        assert_eq!(public_user.id, user.id);
        assert_eq!(public_user.email, user.email);
        assert_eq!(public_user.name, user.name);
        assert_eq!(public_user.is_admin, user.is_admin);
        assert_eq!(public_user.created_at, user.created_at);
        assert_eq!(public_user.updated_at, user.updated_at);

        // Ensure password hash is not accessible in public user
        let serialized = serde_json::to_string(&public_user).expect("Failed to serialize");
        assert!(!serialized.contains("password"));
    }
}
