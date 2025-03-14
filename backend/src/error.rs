use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    // Mainly used in tests - for handling invalid client requests
    #[error("Invalid request: {0}")]
    #[allow(dead_code)]
    BadRequest(String),

    // Mainly used in tests - for handling resources not found
    #[error("Not found: {0}")]
    #[allow(dead_code)]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    // Mainly used in tests - for handling OpenAI API errors
    #[error("OpenAI API error: {0}")]
    #[allow(dead_code)]
    OpenAI(String),

    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),

    #[error("JWT error: {0}")]
    Jwt(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Auth(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OpenAI(_) => StatusCode::BAD_GATEWAY,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Jwt(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": {
                "message": self.to_string(),
                "code": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigError;
    use axum::http::StatusCode;

    // Utility function to convert any error to an AppError::Internal for testing
    fn internal_error<E>(err: E) -> AppError
    where
        E: std::error::Error,
    {
        AppError::Internal(err.to_string())
    }

    #[test]
    fn test_app_error_status_codes() {
        assert_eq!(
            AppError::Auth("test".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::BadRequest("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::Internal("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::OpenAI("test".to_string()).status_code(),
            StatusCode::BAD_GATEWAY
        );
        assert_eq!(
            AppError::Config(ConfigError::EnvVarMissing("test".to_string())).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            AppError::Jwt("test".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn test_internal_error_conversion() {
        let std_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let app_error = internal_error(std_error);

        match app_error {
            AppError::Internal(msg) => assert!(msg.contains("test error")),
            _ => panic!("Expected Internal error variant"),
        }
    }
}
