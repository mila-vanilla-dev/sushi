# TPS Orders API Documentation

This document provides comprehensive documentation for the TPS Orders API, including authentication, user management, and order processing endpoints.

## Table of Contents

- [Overview](#overview)
- [Authentication](#authentication)
- [Base URL](#base-url)
- [Response Formats](#response-formats)
- [Error Handling](#error-handling)
- [Authentication Endpoints](#authentication-endpoints)
- [User Management Endpoints](#user-management-endpoints)
- [Admin Endpoints](#admin-endpoints)
- [Order Endpoints](#order-endpoints)
- [Examples](#examples)

## Overview

The TPS Orders API is a RESTful web service that provides:

- JWT-based authentication and authorization
- User registration and management
- Role-based access control (Admin vs Customer)
- Order processing and UPS shipping integration
- Password security with Argon2id hashing

## Authentication

The API uses JWT (JSON Web Tokens) for authentication. Include the token in the `Authorization` header:

```
Authorization: Bearer <your-jwt-token>
```

### Token Structure

JWT tokens contain the following claims:

- `sub` - User ID (UUID)
- `email` - User email address
- `name` - User full name
- `admin` - Boolean indicating admin privileges
- `exp` - Expiration timestamp
- `iat` - Issued at timestamp

### Token Expiration

Tokens expire after 24 hours by default. Refresh tokens by logging in again.

## Base URL

```
http://localhost:3000/api
```

## Response Formats

### Success Response Format

```json
{
  "user": {
    "id": "uuid",
    "email": "string",
    "name": "string",
    "created_at": "timestamp",
    "updated_at": "timestamp",
    "is_admin": "boolean"
  },
  "message": "string"
}
```

### Token Response Format

```json
{
  "token": "jwt-string",
  "expires_in": 86400,
  "token_type": "Bearer"
}
```

### Error Response Format

```json
{
  "message": "Error description"
}
```

## Error Handling

The API uses standard HTTP status codes:

| Status Code | Description |
|-------------|-------------|
| 200 | OK - Request successful |
| 400 | Bad Request - Invalid input data |
| 401 | Unauthorized - Authentication required or invalid |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource not found |
| 500 | Internal Server Error - Server error |

______________________________________________________________________

# Authentication Endpoints

## Register User

Create a new user account. All public registrations create customer accounts.

**Endpoint:** `POST /auth/register`\
**Authentication:** None required\
**Content-Type:** `application/json`

### Request Body

```json
{
  "email": "string",           // Required: Valid email address
  "name": "string",            // Required: User's full name
  "password": "string"         // Required: Strong password (8+ chars, mixed case, digits, special chars)
}
```

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:11:48.331498874Z",
    "is_admin": false
  },
  "token": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "expires_in": 86400,
    "token_type": "Bearer"
  },
  "message": "User registered successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 400 | "User with this email already exists" |
| 400 | "Password must be at least 8 characters long" |
| 400 | "Email must contain @ symbol" |

### Example

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "name": "John Doe",
    "password": "SecurePass123!",
    "role": "customer"
  }'
```

______________________________________________________________________

## Login User

Authenticate user credentials and receive a JWT token.

**Endpoint:** `POST /auth/login`\
**Authentication:** None required\
**Content-Type:** `application/json`

### Request Body

```json
{
  "email": "string",    // Required: User's email address
  "password": "string"  // Required: User's password
}
```

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:11:48.331498874Z",
    "is_admin": false
  },
  "token": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "expires_in": 86400,
    "token_type": "Bearer"
  },
  "message": "Login successful"
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Invalid email or password" |

### Example

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123!"
  }'
```

______________________________________________________________________

## Logout User

Invalidate the current session (client-side token removal).

**Endpoint:** `POST /auth/logout`\
**Authentication:** None required

### Response

**Status:** `200 OK`

```json
{
  "message": "Logged out successfully. Please discard your token."
}
```

### Example

```bash
curl -X POST http://localhost:3000/api/auth/logout
```

______________________________________________________________________

## Get Current User

Retrieve the authenticated user's profile information.

**Endpoint:** `GET /auth/me`\
**Authentication:** Required (JWT token)

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:11:48.331498874Z",
    "is_admin": false
  },
  "message": "User profile retrieved successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Unauthorized" |

### Example

```bash
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

______________________________________________________________________

## Forgot Password

Generate a password reset token for the specified email address.

**Endpoint:** `POST /auth/forgot-password`\
**Authentication:** None required\
**Content-Type:** `application/json`

### Request Body

```json
{
  "email": "string"  // Required: User's email address
}
```

### Response

**Status:** `200 OK`

```json
{
  "message": "Password reset instructions have been sent to your email"
}
```

### Errors

| Status | Message |
|--------|---------|
| 404 | "User not found" |

### Example

```bash
curl -X POST http://localhost:3000/api/auth/forgot-password \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

______________________________________________________________________

## Reset Password

Reset user password using a valid reset token.

**Endpoint:** `POST /auth/reset-password`\
**Authentication:** None required\
**Content-Type:** `application/json`

### Request Body

```json
{
  "token": "string",         // Required: Password reset token
  "new_password": "string"   // Required: New password (must meet strength requirements)
}
```

### Response

**Status:** `200 OK`

```json
{
  "message": "Password reset successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 400 | "Invalid or expired reset token" |
| 400 | "Reset token has expired" |
| 400 | Password validation errors |

### Example

```bash
curl -X POST http://localhost:3000/api/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{
    "token": "reset-token-uuid",
    "new_password": "NewSecurePass123!"
  }'
```

______________________________________________________________________

# User Management Endpoints

## Get User by ID

Retrieve a user's profile by their ID. Users can access their own profile, admins can access any profile.

**Endpoint:** `GET /users/{id}`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin or self-access only

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| id | UUID | User's unique identifier |

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:11:48.331498874Z",
    "is_admin": false
  },
  "message": "User retrieved successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Unauthorized" |
| 403 | "Access denied" |
| 404 | "User not found" |

### Example

```bash
curl -X GET http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

______________________________________________________________________

## Update User Profile

Update a user's profile information. Users can update their own profile, admins can update any profile.

**Endpoint:** `PATCH /users/{id}`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin or self-access only\
**Content-Type:** `application/json`

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| id | UUID | User's unique identifier |

### Request Body

```json
{
  "name": "string",   // Optional: New user name
  "email": "string"   // Optional: New email address
}
```

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "newemail@example.com",
    "name": "John Smith",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:15:22.445231876Z",
    "is_admin": false
  },
  "message": "Profile updated successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 400 | "Email already in use" |
| 401 | "Unauthorized" |
| 403 | "Access denied" |
| 404 | "User not found" |

### Example

```bash
curl -X PATCH http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Smith",
    "email": "john.smith@example.com"
  }'
```

______________________________________________________________________

## Update User Password

Update a user's password. Users can only update their own password.

**Endpoint:** `PATCH /users/{id}/password`\
**Authentication:** Required (JWT token)\
**Authorization:** Self-access only\
**Content-Type:** `application/json`

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| id | UUID | User's unique identifier |

### Request Body

```json
{
  "current_password": "string",  // Required: Current password for verification
  "new_password": "string"       // Required: New password (must meet strength requirements)
}
```

### Response

**Status:** `200 OK`

```json
{
  "message": "Password updated successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 400 | "Current password is incorrect" |
| 400 | Password validation errors |
| 401 | "Unauthorized" |
| 403 | "Access denied" |

### Example

```bash
curl -X PATCH http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45/password \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "current_password": "OldPassword123!",
    "new_password": "NewSecurePass456!"
  }'
```

______________________________________________________________________

## Delete User Account

Delete a user account. Users can delete their own account, admins can delete any account.

**Endpoint:** `DELETE /users/{id}`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin or self-access only

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| id | UUID | User's unique identifier |

### Response

**Status:** `200 OK`

```json
{
  "message": "User deleted successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Unauthorized" |
| 403 | "Access denied" |
| 404 | "User not found" |

### Example

```bash
curl -X DELETE http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45 \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

______________________________________________________________________

# Admin Endpoints

## List All Users

Retrieve a list of all users in the system. Admin access required.

**Endpoint:** `GET /users`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin only

### Response

**Status:** `200 OK`

```json
{
  "users": [
    {
      "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
      "email": "user@example.com",
      "name": "John Doe",
      "created_at": "2025-08-13T03:11:48.331493287Z",
      "updated_at": "2025-08-13T03:11:48.331498874Z",
      "is_admin": false
    },
    {
      "id": "6459b8ee-b962-4cc9-aba2-ac8bb3c5e15e",
      "email": "admin@example.com",
      "name": "Admin User",
      "created_at": "2025-08-13T03:12:37.755318884Z",
      "updated_at": "2025-08-13T03:12:37.755328453Z",
      "is_admin": true
    }
  ],
  "total": 2
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Unauthorized" |
| 403 | "Forbidden" (non-admin access) |

### Example

```bash
curl -X GET http://localhost:3000/api/users \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```

______________________________________________________________________

## Update User Role

Update a user's role (promote/demote admin privileges). Admin access required.

**Endpoint:** `PATCH /users/{id}/role`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin only\
**Content-Type:** `application/json`

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| id | UUID | User's unique identifier |

### Request Body

```json
{
  "is_admin": boolean  // Required: true to grant admin privileges, false to revoke
}
```

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:20:15.123456789Z",
    "is_admin": true
  },
  "message": "Role updated successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 401 | "Unauthorized" |
| 403 | "Forbidden" (non-admin access) |
| 404 | "User not found" |

### Example

```bash
# Promote user to admin
curl -X PATCH http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45/role \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{"is_admin": true}'

# Revoke admin privileges
curl -X PATCH http://localhost:3000/api/users/1ee3275f-4340-49c1-af8f-3bb31cde8f45/role \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{"is_admin": false}'
```

______________________________________________________________________

## Create Admin User

Create a new admin user account. Only existing admins can create other admins.

**Endpoint:** `POST /admin/create-admin`\
**Authentication:** Required (JWT token)\
**Authorization:** Admin only\
**Content-Type:** `application/json`

### Request Body

```json
{
  "email": "string",           // Required: Valid email address
  "name": "string",            // Required: User's full name
  "password": "string"         // Required: Strong password (8+ chars, mixed case, digits, special chars)
}
```

### Response

**Status:** `200 OK`

```json
{
  "user": {
    "id": "1ee3275f-4340-49c1-af8f-3bb31cde8f45",
    "email": "admin@example.com",
    "name": "Admin User",
    "created_at": "2025-08-13T03:11:48.331493287Z",
    "updated_at": "2025-08-13T03:11:48.331498874Z",
    "is_admin": true
  },
  "message": "Admin user created successfully"
}
```

### Errors

| Status | Message |
|--------|---------|
| 400 | "User already exists" |
| 400 | "Email validation failed: [reason]" |
| 400 | "Password validation failed: [reason]" |
| 401 | "Unauthorized" |
| 403 | "Forbidden" (non-admin access) |

### Example

```bash
# Create admin user
curl -X POST http://localhost:3000/api/admin/create-admin \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newadmin@example.com",
    "name": "New Admin",
    "password": "SecurePass123!"
  }'
```

______________________________________________________________________

# Order Endpoints

## Create Order

Create a new print order with customer information and shipping details.

**Endpoint:** `POST /orders`\
**Authentication:** Required (JWT token)\
**Content-Type:** `application/json`

### Request Body

*Note: Full order schema documentation would be defined based on the Order model implementation.*

### Response

**Status:** `200 OK`

*Note: Response schema would be defined based on the orders endpoint implementation.*

### Example

```bash
curl -X POST http://localhost:3000/api/orders \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  -H "Content-Type: application/json" \
  -d '{ ... order data ... }'
```

______________________________________________________________________

# Examples

## Complete Authentication Flow

### 1. Register a new user

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "customer@example.com",
    "name": "Jane Customer",
    "password": "SecurePass123!"
  }'
```

### 2. Login and get token

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "customer@example.com",
    "password": "SecurePass123!"
  }'
```

### 3. Use token to access protected endpoints

```bash
export JWT_TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."

curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer $JWT_TOKEN"
```

### 4. Update profile

```bash
curl -X PATCH http://localhost:3000/api/users/your-user-id \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "email": "jane.smith@example.com"
  }'
```

## Admin Operations

### 1. Register admin user

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "name": "System Admin",
    "password": "AdminPass123!",
    "role": "admin"
  }'
```

### 2. List all users (admin only)

```bash
export ADMIN_TOKEN="admin-jwt-token-here"

curl -X GET http://localhost:3000/api/users \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

### 3. Promote user to admin

```bash
curl -X PATCH http://localhost:3000/api/users/user-id-here/role \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"is_admin": true}'
```

## Password Management

### 1. Request password reset

```bash
curl -X POST http://localhost:3000/api/auth/forgot-password \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

### 2. Reset password with token

```bash
curl -X POST http://localhost:3000/api/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{
    "token": "reset-token-from-email",
    "new_password": "NewSecurePass456!"
  }'
```

### 3. Change password (authenticated)

```bash
curl -X PATCH http://localhost:3000/api/users/your-user-id/password \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "current_password": "OldPassword123!",
    "new_password": "NewPassword456!"
  }'
```

______________________________________________________________________

## Security Notes

### Password Requirements

Passwords must meet the following criteria:

- Minimum 8 characters
- Maximum 128 characters
- At least one uppercase letter
- At least one lowercase letter
- At least one digit
- At least one special character (`!@#$%^&*()_+-=[]{}|;:,.<>?`)

### JWT Security

- Tokens are signed using HS256 algorithm
- Set `JWT_SECRET` environment variable for production
- Tokens expire after 24 hours
- Include tokens in `Authorization: Bearer <token>` header

### Rate Limiting

*Note: Rate limiting is not currently implemented but should be added for production use.*

### HTTPS

*Note: Use HTTPS in production to protect authentication tokens in transit.*

______________________________________________________________________

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `JWT_SECRET` | Secret key for JWT token signing | Development default (insecure) |
| `UPS_CLIENT_ID` | UPS API client ID | - |
| `UPS_CLIENT_SECRET` | UPS API client secret | - |

## Development Setup

1. Set environment variables in `.env` file
1. Start the server: `cargo run`
1. Server runs on `http://localhost:3000`
1. Health check: `GET http://localhost:3000/`

______________________________________________________________________

*This documentation is for TPS Orders API v0.1.0*
