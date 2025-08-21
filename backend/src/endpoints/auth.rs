use crate::{
    AppState,
    auth::{Claims, TokenResponse, generate_token},
    models::user::{PublicUser, User},
};
use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Request payload for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

/// Request payload for admin user creation (admin only)
#[derive(Debug, Deserialize)]
pub struct CreateAdminRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

/// Request payload for user login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Request payload for password update
#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Request payload for user profile update
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Request payload for role update (admin only)
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub is_admin: bool,
}

/// Request payload for password reset
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Request payload for password reset confirmation
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

/// User role enumeration
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Customer,
}

/// Response for successful authentication
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: PublicUser,
    pub token: TokenResponse,
    pub message: String,
}

/// Response for user operations
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: PublicUser,
    pub message: String,
}

/// Response for user listing (admin only)
#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    pub users: Vec<PublicUser>,
    pub total: usize,
}

/// Simple response message
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Simple in-memory user store for demonstration
/// TODO: Move users to PostgreSQL
#[derive(Debug, Default)]
pub struct UserStore {
    users: HashMap<String, User>, // email -> user
    password_reset_tokens: HashMap<String, (String, chrono::DateTime<chrono::Utc>)>, // token -> (email, expiry)
}

impl UserStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a bootstrap admin user for testing
    pub fn new_with_admin() -> Self {
        let mut store = Self::new();

        // Create a bootstrap admin user
        // let admin_request = CreateAdminRequest {
        //     email: "admin@example.com".to_string(),
        //     name: "Bootstrap Admin".to_string(),
        //     password: "AdminPass123!".to_string(),
        // };

        // Get bootstrap admin details from env
        let name = std::env::var("BOOTSTRAP_ADMIN_NAME").expect("BOOTSTRAP_ADMIN_NAME must be set");
        let email =
            std::env::var("BOOTSTRAP_ADMIN_EMAIL").expect("BOOTSTRAP_ADMIN_EMAIL must be set");
        let password = std::env::var("BOOTSTRAP_ADMIN_PASSWORD")
            .expect("BOOTSTRAP_ADMIN_PASSWORD must be set");

        let admin_request = CreateAdminRequest {
            email,
            name,
            password,
        };

        tracing::info!("Bootstrap admin user created: {}", admin_request.name);

        // Log other fields, but only in debug builds
        #[cfg(debug_assertions)]
        {
            tracing::info!(
                "Bootstrap admin email: {}, password: {}",
                admin_request.email,
                admin_request.password
            );
        }

        // Use the create_admin method directly
        let _ = store.create_admin(admin_request);

        store
    }

    /// Register a new user (always creates a customer)
    pub fn register(&mut self, request: RegisterRequest) -> Result<AuthResponse, String> {
        // Check if user already exists
        if self.users.contains_key(&request.email) {
            return Err("User with this email already exists".to_string());
        }

        // Create new user (includes validation) - always a customer
        let user = User::new(request.email.clone(), request.name, &request.password)
            .map_err(|e| e.to_string())?;

        let public_user = user.to_public();

        // Generate JWT token
        let token = generate_token(
            user.id,
            &user.email,
            &user.name,
            user.is_admin,
            None, // Use default expiration
        )
        .map_err(|e| format!("Failed to generate token: {}", e))?;

        // Store user
        self.users.insert(request.email, user);

        Ok(AuthResponse {
            user: public_user,
            token,
            message: "User registered successfully".to_string(),
        })
    }

    /// Create admin user (admin only operation)
    pub fn create_admin(&mut self, request: CreateAdminRequest) -> Result<UserResponse, String> {
        // Check if user already exists
        if self.users.contains_key(&request.email) {
            return Err("User with this email already exists".to_string());
        }

        // Create new admin user (includes validation)
        let mut user = User::new(request.email.clone(), request.name, &request.password)
            .map_err(|e| e.to_string())?;

        // Set admin role
        user.set_admin(true);

        let public_user = user.to_public();

        // Store user
        self.users.insert(request.email, user);

        Ok(UserResponse {
            user: public_user,
            message: "Admin user created successfully".to_string(),
        })
    }

    /// Authenticate a user login
    pub fn login(&self, request: LoginRequest) -> Result<AuthResponse, String> {
        // Find user by email
        let user = self
            .users
            .get(&request.email)
            .ok_or("Invalid email or password".to_string())?;

        // Verify password
        let is_valid = user
            .verify_password(&request.password)
            .map_err(|e| format!("Authentication error: {}", e))?;

        if !is_valid {
            return Err("Invalid email or password".to_string());
        }

        // Generate JWT token
        let token = generate_token(
            user.id,
            &user.email,
            &user.name,
            user.is_admin,
            None, // Use default expiration
        )
        .map_err(|e| format!("Failed to generate token: {}", e))?;

        Ok(AuthResponse {
            user: user.to_public(),
            token,
            message: "Login successful".to_string(),
        })
    }

    /// Get user by ID
    pub fn get_user_by_id(&self, user_id: &Uuid) -> Option<&User> {
        self.users.values().find(|user| &user.id == user_id)
    }

    /// Get user by email
    pub fn get_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.get(email)
    }

    /// Update user profile
    pub fn update_user(
        &mut self,
        user_id: &Uuid,
        request: UpdateProfileRequest,
    ) -> Result<UserResponse, String> {
        // If email is being updated, check for conflicts first
        if let Some(ref new_email) = request.email {
            // Find the current user's email
            let current_email = self
                .users
                .iter()
                .find(|(_, user)| &user.id == user_id)
                .map(|(email, _)| email.clone())
                .ok_or("User not found".to_string())?;

            if new_email != &current_email && self.users.contains_key(new_email) {
                return Err("Email already in use".to_string());
            }
        }

        // Find user by ID and get their current email
        let (old_email, user_exists) = self
            .users
            .iter()
            .find(|(_, user)| &user.id == user_id)
            .map(|(email, _)| (email.clone(), true))
            .ok_or("User not found".to_string())?;

        if !user_exists {
            return Err("User not found".to_string());
        }

        // Update the user
        let user = self.users.get_mut(&old_email).unwrap();
        user.update(request.email.clone(), request.name);

        // If email changed, update the HashMap key
        if let Some(new_email) = request.email
            && new_email != old_email
        {
            let user = self.users.remove(&old_email).unwrap();
            self.users.insert(new_email.clone(), user);

            let user = self.users.get(&new_email).unwrap();
            return Ok(UserResponse {
                user: user.to_public(),
                message: "Profile updated successfully".to_string(),
            });
        }

        let user = self.users.get(&old_email).unwrap();
        Ok(UserResponse {
            user: user.to_public(),
            message: "Profile updated successfully".to_string(),
        })
    }

    /// Update user password
    pub fn update_password(
        &mut self,
        user_id: &Uuid,
        request: UpdatePasswordRequest,
    ) -> Result<MessageResponse, String> {
        // Find user by ID
        let user = self
            .users
            .values_mut()
            .find(|user| &user.id == user_id)
            .ok_or("User not found".to_string())?;

        // Verify current password
        let is_valid = user
            .verify_password(&request.current_password)
            .map_err(|e| format!("Authentication error: {}", e))?;

        if !is_valid {
            return Err("Current password is incorrect".to_string());
        }

        // Update password
        user.update_password(&request.new_password)
            .map_err(|e| e.to_string())?;

        Ok(MessageResponse {
            message: "Password updated successfully".to_string(),
        })
    }

    /// Delete user (admin or self-access)
    pub fn delete_user(&mut self, user_id: &Uuid) -> Result<MessageResponse, String> {
        // Find and remove user
        let user_email = self
            .users
            .iter()
            .find(|(_, user)| &user.id == user_id)
            .map(|(email, _)| email.clone())
            .ok_or("User not found".to_string())?;

        self.users.remove(&user_email);

        Ok(MessageResponse {
            message: "User deleted successfully".to_string(),
        })
    }

    /// Update user role (admin only)
    pub fn update_user_role(
        &mut self,
        user_id: &Uuid,
        request: UpdateRoleRequest,
    ) -> Result<UserResponse, String> {
        // Find user by ID
        let user = self
            .users
            .values_mut()
            .find(|user| &user.id == user_id)
            .ok_or("User not found".to_string())?;

        user.set_admin(request.is_admin);

        Ok(UserResponse {
            user: user.to_public(),
            message: "Role updated successfully".to_string(),
        })
    }

    /// List all users (admin only)
    pub fn list_users(&self) -> UsersListResponse {
        let users: Vec<PublicUser> = self.users.values().map(|user| user.to_public()).collect();

        UsersListResponse {
            total: users.len(),
            users,
        }
    }

    /// Generate password reset token
    pub fn generate_password_reset_token(&mut self, email: &str) -> Result<String, String> {
        // Check if user exists
        if !self.users.contains_key(email) {
            return Err("User not found".to_string());
        }

        // Generate reset token
        let token = Uuid::new_v4().to_string();
        let expiry = chrono::Utc::now() + chrono::Duration::hours(1); // 1 hour expiry

        self.password_reset_tokens
            .insert(token.clone(), (email.to_string(), expiry));

        Ok(token)
    }

    /// Reset password with token
    pub fn reset_password(
        &mut self,
        request: ResetPasswordRequest,
    ) -> Result<MessageResponse, String> {
        // Validate token
        let (email, expiry) = self
            .password_reset_tokens
            .get(&request.token)
            .ok_or("Invalid or expired reset token".to_string())?
            .clone();

        // Check if token is expired
        if chrono::Utc::now() > expiry {
            self.password_reset_tokens.remove(&request.token);
            return Err("Reset token has expired".to_string());
        }

        // Find user and update password
        let user = self
            .users
            .get_mut(&email)
            .ok_or("User not found".to_string())?;

        user.update_password(&request.new_password)
            .map_err(|e| e.to_string())?;

        // Remove used token
        self.password_reset_tokens.remove(&request.token);

        Ok(MessageResponse {
            message: "Password reset successfully".to_string(),
        })
    }
}

/// POST /api/auth/register
pub async fn register_endpoint(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<MessageResponse>)> {
    let mut user_store = state.user_store.write().await;

    match user_store.register(request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// POST /api/auth/login
pub async fn login_endpoint(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<MessageResponse>)> {
    let user_store = state.user_store.read().await;

    match user_store.login(request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// POST /api/auth/logout
pub async fn logout_endpoint() -> Json<MessageResponse> {
    // In a stateless JWT system, logout is handled client-side by discarding the token
    // For more security, you could implement a token blacklist
    Json(MessageResponse {
        message: "Logged out successfully. Please discard your token.".to_string(),
    })
}

/// GET /api/auth/me
pub async fn me_endpoint(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, (StatusCode, Json<MessageResponse>)> {
    let user_store = state.user_store.read().await;
    let user_id = claims.sub.parse::<Uuid>().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Invalid user ID".to_string(),
            }),
        )
    })?;

    let user = user_store.get_user_by_id(&user_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "User not found".to_string(),
            }),
        )
    })?;

    Ok(Json(UserResponse {
        user: user.to_public(),
        message: "User profile retrieved successfully".to_string(),
    }))
}

/// GET /api/users/:id
pub async fn get_user_endpoint(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, (StatusCode, Json<MessageResponse>)> {
    let current_user_id = claims.sub.parse::<Uuid>().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Invalid user ID".to_string(),
            }),
        )
    })?;

    // Check if user is admin or accessing their own profile
    if !claims.admin && current_user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(MessageResponse {
                message: "Access denied".to_string(),
            }),
        ));
    }

    let user_store = state.user_store.read().await;
    let user = user_store.get_user_by_id(&user_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "User not found".to_string(),
            }),
        )
    })?;

    Ok(Json(UserResponse {
        user: user.to_public(),
        message: "User retrieved successfully".to_string(),
    }))
}

/// PATCH /api/users/:id
pub async fn update_user_endpoint(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(update_request): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<MessageResponse>)> {
    let current_user_id = claims.sub.parse::<Uuid>().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Invalid user ID".to_string(),
            }),
        )
    })?;

    // Check if user is admin or updating their own profile
    if !claims.admin && current_user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(MessageResponse {
                message: "Access denied".to_string(),
            }),
        ));
    }

    let mut user_store = state.user_store.write().await;
    match user_store.update_user(&user_id, update_request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// PATCH /api/users/:id/password
pub async fn update_password_endpoint(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(password_request): Json<UpdatePasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let current_user_id = claims.sub.parse::<Uuid>().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Invalid user ID".to_string(),
            }),
        )
    })?;

    // Users can only update their own password
    if current_user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(MessageResponse {
                message: "Access denied".to_string(),
            }),
        ));
    }

    let mut user_store = state.user_store.write().await;
    match user_store.update_password(&user_id, password_request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// DELETE /api/users/:id
pub async fn delete_user_endpoint(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let current_user_id = claims.sub.parse::<Uuid>().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Invalid user ID".to_string(),
            }),
        )
    })?;

    // Check if user is admin or deleting their own account
    if !claims.admin && current_user_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(MessageResponse {
                message: "Access denied".to_string(),
            }),
        ));
    }

    let mut user_store = state.user_store.write().await;
    match user_store.delete_user(&user_id) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// GET /api/users (admin only)
pub async fn list_users_endpoint(State(state): State<AppState>) -> Json<UsersListResponse> {
    let user_store = state.user_store.read().await;
    Json(user_store.list_users())
}

/// PATCH /api/users/:id/role (admin only)
pub async fn update_user_role_endpoint(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(role_request): Json<UpdateRoleRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<MessageResponse>)> {
    let mut user_store = state.user_store.write().await;
    match user_store.update_user_role(&user_id, role_request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// POST /api/auth/forgot-password
pub async fn forgot_password_endpoint(
    State(state): State<AppState>,
    Json(request): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let mut user_store = state.user_store.write().await;

    match user_store.generate_password_reset_token(&request.email) {
        Ok(token) => {
            // In a real application, you would send this token via email
            tracing::info!("Password reset token for {}: {}", request.email, token);

            Ok(Json(MessageResponse {
                message: "Password reset instructions have been sent to your email".to_string(),
            }))
        }
        Err(error) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse { message: error }),
        )),
    }
}

/// POST /api/auth/reset-password
pub async fn reset_password_endpoint(
    State(state): State<AppState>,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<MessageResponse>)> {
    let mut user_store = state.user_store.write().await;

    match user_store.reset_password(request) {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(MessageResponse { message: error }),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_registration() {
        let mut store = UserStore::new();

        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "SecurePass123!".to_string(),
        };

        let response = store
            .register(request)
            .expect("Registration should succeed");
        assert_eq!(response.user.email, "test@example.com");
        assert_eq!(response.user.name, "Test User");
        assert!(!response.user.is_admin);
    }

    #[test]
    fn test_admin_creation() {
        let mut store = UserStore::new();

        let request = CreateAdminRequest {
            email: "admin@example.com".to_string(),
            name: "Admin User".to_string(),
            password: "SecurePass123!".to_string(),
        };

        let response = store
            .create_admin(request)
            .expect("Admin creation should succeed");
        assert_eq!(response.user.email, "admin@example.com");
        assert_eq!(response.user.name, "Admin User");
        assert!(response.user.is_admin);
    }

    #[test]
    fn test_login() {
        let mut store = UserStore::new();

        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password: "SecurePass123!".to_string(),
        };

        store
            .register(register_request)
            .expect("Registration should succeed");

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "SecurePass123!".to_string(),
        };

        let response = store.login(login_request).expect("Login should succeed");
        assert_eq!(response.user.email, "test@example.com");
        assert!(!response.token.token.is_empty());
    }
}
