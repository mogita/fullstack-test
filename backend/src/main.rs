use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tracing::{error, info};

use crate::api::create_router;
use crate::config::Config;
use crate::error::AppError;

mod api;
mod auth;
mod config;
mod error;
mod models;
mod openai;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = match Config::load() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            Arc::new(config)
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e.into());
        }
    };

    // Create the application router
    let app = create_router(config.clone());

    // Bind to the configured address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Listening on {}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| AppError::Internal(format!("Server error: {}", e)))
}
