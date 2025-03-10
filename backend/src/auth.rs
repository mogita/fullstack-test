use std::sync::Arc;

use axum::extract::State;
use axum::headers::{authorization::Bearer, Authorization, Cookie};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Json, TypedHeader};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use tracing::{debug, info};

use crate::config::Config;
use crate::error::AppError;
use crate::models::{Claims, LoginRequest, LoginResponse};

// Hardcoded user credentials as specified in the requirements
const USERNAME: &str = "neo";
const PASSWORD: &str = "script-chairman-fondly-yippee";

// Generate a JWT token for a user
pub fn generate_token(
    username: &str,
    config: &Config,
) -> Result<(String, chrono::DateTime<Utc>), AppError> {
    let now = Utc::now();
    let expires_at = now + Duration::seconds(config.jwt.expiration);
    let exp = expires_at.timestamp();
    let iat = now.timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::Jwt(format!("Failed to create token: {}", e)))?;

    Ok((token, expires_at))
}

// Validate a JWT token
pub fn validate_token(token: &str, config: &Config) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Jwt(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims)
}

// Login handler
pub async fn login(
    State(config): State<Arc<Config>>,
    Json(login_req): Json<LoginRequest>,
) -> Result<(axum::http::HeaderMap, Json<LoginResponse>), AppError> {
    // Check if the username matches our hardcoded user
    if login_req.username != USERNAME {
        return Err(AppError::Auth("Invalid username or password".to_string()));
    }

    // Check if the password matches our hardcoded password
    if login_req.password != PASSWORD {
        return Err(AppError::Auth("Invalid username or password".to_string()));
    }

    // Generate a token
    let (token, expires_at) = generate_token(&login_req.username, &config)?;

    // Set cookie in response headers
    let mut headers = axum::http::HeaderMap::new();

    // Build cookie with appropriate security settings from config
    let mut cookie_value = format!(
        "auth_token={}; Path=/; SameSite={}",
        token, config.cookie.same_site
    );

    // Add Secure flag if configured
    if config.cookie.secure {
        cookie_value.push_str("; Secure");
    }

    // Add domain if configured
    if let Some(domain_value) = &config.cookie.domain {
        cookie_value.push_str(&format!("; Domain={}", domain_value));
    }

    // Set cookie header
    headers.insert(
        axum::http::header::SET_COOKIE,
        axum::http::HeaderValue::from_str(&cookie_value)
            .map_err(|e| AppError::Internal(format!("Failed to create cookie header: {}", e)))?,
    );

    info!("User {} logged in successfully", login_req.username);
    info!("Set cookie: {}", cookie_value);

    Ok((headers, Json(LoginResponse { token, expires_at })))
}

// Helper function to extract token from cookies
fn extract_token_from_cookies(cookies: &str) -> Option<String> {
    cookies
        .split(';')
        .map(|c| c.trim())
        .find(|c| c.starts_with("auth_token="))
        .map(|c| c[11..].to_string())
}

// Authentication middleware
pub async fn auth_middleware<B>(
    State(config): State<Arc<Config>>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    cookies_header: Option<TypedHeader<Cookie>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    // Debug headers
    if let Some(cookie_header) = req.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            debug!("Received Cookie header: {}", cookie_str);
        }
    }

    // Try to get token from Authorization header
    let token = if let Some(TypedHeader(auth)) = auth_header {
        debug!("Using Authorization Bearer token");
        auth.token().to_string()
    } else if let Some(TypedHeader(cookies)) = cookies_header {
        // Try to get token from cookies - accessing cookie values directly
        match cookies.get("auth_token") {
            Some(token) => {
                debug!("Found auth_token in Cookie typed header: {}", token);
                token.to_string()
            }
            None => {
                debug!(
                    "No auth_token in Cookie typed header. Available cookies: {:?}",
                    cookies.iter().map(|(name, _)| name).collect::<Vec<_>>()
                );
                // Check for Cookie header as raw header (fallback)
                if let Some(cookie_header) = req.headers().get("Cookie") {
                    if let Ok(cookie_str) = cookie_header.to_str() {
                        match extract_token_from_cookies(cookie_str) {
                            Some(token) => {
                                debug!("Extracted auth_token from raw Cookie header: {}", token);
                                token
                            }
                            None => {
                                return Err(AppError::Auth("Authentication required".to_string()))
                            }
                        }
                    } else {
                        return Err(AppError::Auth("Authentication required".to_string()));
                    }
                } else {
                    return Err(AppError::Auth("Authentication required".to_string()));
                }
            }
        }
    } else {
        // Check for Cookie header as raw header (fallback)
        if let Some(cookie_header) = req.headers().get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                match extract_token_from_cookies(cookie_str) {
                    Some(token) => {
                        debug!("Extracted auth_token from raw Cookie header: {}", token);
                        token
                    }
                    None => {
                        debug!("No auth_token found in Cookie header: {}", cookie_str);
                        return Err(AppError::Auth("Authentication required".to_string()));
                    }
                }
            } else {
                return Err(AppError::Auth("Authentication required".to_string()));
            }
        } else {
            debug!("No Cookie header found in request");
            return Err(AppError::Auth("Authentication required".to_string()));
        }
    };

    // Validate the token
    let claims = validate_token(&token, &config)?;

    // Check if the token is expired
    let now = Utc::now().timestamp();
    if claims.exp < now {
        return Err(AppError::Auth("Token expired".to_string()));
    }

    debug!("Authenticated user: {}", claims.sub);

    // Continue with the request
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
    use rand::rngs::OsRng;

    // Generate a password hash for testing
    fn generate_password_hash(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
    }

    // Verify a password against a hash for testing
    fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Failed to parse hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    #[test]
    fn test_password_hash_and_verify() {
        let password = "test-password";
        let hash = generate_password_hash(password).unwrap();

        let is_valid = verify_password(password, &hash).unwrap();
        assert!(is_valid);

        let is_invalid = verify_password("wrong-password", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_token_generation_and_validation() {
        let config = Config::default_test_config();
        let username = "test-user";

        let (token, _) = generate_token(username, &config).unwrap();
        let claims = validate_token(&token, &config).unwrap();

        assert_eq!(claims.sub, username);
    }

    #[test]
    fn test_token_expiration() {
        let mut config = Config::default_test_config();
        config.jwt.expiration = -10; // Set expiration to the past

        let username = "test-user";
        let (token, _) = generate_token(username, &config).unwrap();

        // Token should be expired
        let now = Utc::now().timestamp();
        let claims = validate_token(&token, &config).unwrap();
        assert!(claims.exp < now);
    }
}
