use std::sync::Arc;

use axum::extract::State;
use axum::http::Method;
use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::auth::{auth_middleware, login};
use crate::config::Config;
use crate::openai::{expand, paraphrase, summarize, translate};

// Health check handler
async fn health_check() -> &'static str {
    "OK"
}

// Create the application router
pub fn create_router(config: Arc<Config>) -> Router {
    // Define CORS configuration
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Public routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/login", post(login));

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/api/text/paraphrase", post(paraphrase))
        .route("/api/text/expand", post(expand))
        .route("/api/text/summarize", post(summarize))
        .route("/api/text/translate", post(translate))
        .layer(middleware::from_fn_with_state(
            config.clone(),
            auth_middleware,
        ));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(config);

    info!("Router configured successfully");
    app
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let config = Arc::new(crate::config::Config::default_test_config());
        let app = create_router(config);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"OK");
    }

    // TODO: Add more comprehensive API tests
    // This would require mocking the authentication and OpenAI services
}
