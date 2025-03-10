use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Authentication models
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (username)
    pub exp: i64,    // Expiration time (as UTC timestamp)
    pub iat: i64,    // Issued at (as UTC timestamp)
}

// Text processing models
#[derive(Debug, Serialize, Deserialize)]
pub struct TextRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub target_language: TargetLanguage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetLanguage {
    English,
    Spanish,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextResponse {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSEEvent {
    pub event: String,
    pub data: String,
}

// For testing purposes
#[cfg(test)]
impl User {
    pub fn new_test_user() -> Self {
        User {
            username: "neo".to_string(),
            // This is not the actual hash - it will be generated in the auth module
            password_hash: "hashed_password".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User {
            username: "test_user".to_string(),
            password_hash: "hashed_password".to_string(),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.username, deserialized.username);
        assert_eq!(user.password_hash, deserialized.password_hash);
    }

    #[test]
    fn test_login_request_serialization() {
        let login = LoginRequest {
            username: "test_user".to_string(),
            password: "password123".to_string(),
        };

        let serialized = serde_json::to_string(&login).unwrap();
        let deserialized: LoginRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(login.username, deserialized.username);
        assert_eq!(login.password, deserialized.password);
    }

    #[test]
    fn test_target_language_serialization() {
        let lang = TargetLanguage::English;

        let serialized = serde_json::to_string(&lang).unwrap();
        assert_eq!(serialized, "\"english\"");

        let deserialized: TargetLanguage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            TargetLanguage::English => {}
            _ => panic!("Expected English variant"),
        }
    }
}
