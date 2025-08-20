//! Authentication middleware for protecting routes

use crate::auth::{Claims, extract_token_from_header, validate_token};
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Extract and validate JWT token from request
pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Extract token from header
    let token = extract_token_from_header(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token and extract claims
    let claims = validate_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Middleware that requires admin privileges
pub async fn admin_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // First run auth middleware logic
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = extract_token_from_header(auth_header).ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = validate_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if user is admin
    if !claims.admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Extract authenticated user claims from request
pub fn get_current_user(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}
