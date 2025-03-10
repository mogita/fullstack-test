use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

use dotenv::dotenv;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to load .env file: {0}")]
    DotEnvError(#[from] std::io::Error),

    #[error("Missing environment variable: {0}")]
    EnvVarMissing(String),

    #[error("Invalid environment variable: {0}, error: {1}")]
    EnvVarInvalid(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
    pub expiration: i64, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieConfig {
    pub secure: bool,
    pub domain: Option<String>,
    pub same_site: String, // "Strict", "Lax", or "None"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub openai: OpenAIConfig,
    pub jwt: JWTConfig,
    pub cookie: CookieConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file if it exists
        if Path::new(".env").exists() {
            dotenv().ok();
        }

        // Server configuration
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()
            .map_err(|e| ConfigError::EnvVarInvalid("SERVER_PORT".to_string(), e.to_string()))?;

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        // OpenAI configuration
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| ConfigError::EnvVarMissing("OPENAI_API_KEY".to_string()))?;

        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        // JWT configuration
        let secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::EnvVarMissing("JWT_SECRET".to_string()))?;

        let expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // Default to 24 hours
            .parse::<i64>()
            .map_err(|e| ConfigError::EnvVarInvalid("JWT_EXPIRATION".to_string(), e.to_string()))?;

        // Cookie configuration
        let secure = env::var("COOKIE_SECURE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .map_err(|e| ConfigError::EnvVarInvalid("COOKIE_SECURE".to_string(), e.to_string()))?;

        let domain = env::var("COOKIE_DOMAIN").ok();

        let same_site = env::var("COOKIE_SAME_SITE").unwrap_or_else(|_| "None".to_string());
        // Validate SameSite value
        if !["Strict", "Lax", "None"].contains(&same_site.as_str()) {
            return Err(ConfigError::EnvVarInvalid(
                "COOKIE_SAME_SITE".to_string(),
                format!("Must be one of: Strict, Lax, None. Got: {}", same_site),
            ));
        }

        Ok(Config {
            server: ServerConfig { port, host },
            openai: OpenAIConfig {
                api_key,
                base_url,
                model,
            },
            jwt: JWTConfig { secret, expiration },
            cookie: CookieConfig {
                secure,
                domain,
                same_site,
            },
        })
    }

    // For testing purposes
    #[cfg(test)]
    pub fn default_test_config() -> Self {
        Config {
            server: ServerConfig {
                port: 3001,
                host: "127.0.0.1".to_string(),
            },
            openai: OpenAIConfig {
                api_key: "test_api_key".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                model: "gpt-3.5-turbo".to_string(),
            },
            jwt: JWTConfig {
                secret: "test_secret_key_for_testing_purposes_only".to_string(),
                expiration: 86400,
            },
            cookie: CookieConfig {
                secure: false,
                domain: None,
                same_site: "None".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_test_config() {
        let config = Config::default_test_config();
        assert_eq!(config.server.port, 3001);
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.openai.api_key, "test_api_key");
        assert_eq!(config.jwt.expiration, 86400);
    }
}
