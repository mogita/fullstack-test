use std::convert::Infallible;
use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig as ClientConfig, Client};
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::Stream;
use futures_util::StreamExt;
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::config::Config;
use crate::error::AppError;
use crate::models::{TargetLanguage, TextRequest, TranslationRequest};

// Struct to wrap SSE response with no-cache headers
struct SseWithNoCacheHeaders<S>(Sse<S>);

impl<S> IntoResponse for SseWithNoCacheHeaders<S>
where
    S: Stream<Item = Result<Event, Infallible>> + Send + 'static,
{
    fn into_response(self) -> Response {
        let mut response = self.0.into_response();

        // Add headers to prevent caching
        let headers = response.headers_mut();
        headers.insert("Cache-Control", "no-cache, no-transform".parse().unwrap());
        headers.insert("X-Accel-Buffering", "no".parse().unwrap());

        response
    }
}

// Function to create a client for the OpenAI API
fn create_client(config: &Config) -> Client<ClientConfig> {
    let openai_config = ClientConfig::new()
        .with_api_key(&config.openai.api_key)
        .with_api_base(&config.openai.base_url);

    Client::with_config(openai_config)
}

// Paraphrase text - support both GET and POST
pub async fn paraphrase(
    State(config): State<Arc<Config>>,
    text_param: Option<Query<TextRequest>>,
    text_json: Option<Json<TextRequest>>,
) -> Result<impl IntoResponse, AppError> {
    // Extract text from either query parameters or JSON body
    let text = if let Some(query) = text_param {
        query.text.clone()
    } else if let Some(json) = text_json {
        json.text.clone()
    } else {
        return Err(AppError::BadRequest("Text is required".to_string()));
    };

    let prompt = format!(
        "Paraphrase the following text while maintaining its original meaning:\n\n{}",
        text
    );

    process_text_with_openai(config, prompt).await
}

// Expand text - support both GET and POST
pub async fn expand(
    State(config): State<Arc<Config>>,
    text_param: Option<Query<TextRequest>>,
    text_json: Option<Json<TextRequest>>,
) -> Result<impl IntoResponse, AppError> {
    // Extract text from either query parameters or JSON body
    let text = if let Some(query) = text_param {
        query.text.clone()
    } else if let Some(json) = text_json {
        json.text.clone()
    } else {
        return Err(AppError::BadRequest("Text is required".to_string()));
    };

    let prompt = format!(
        "Expand the following text with more details and explanations:\n\n{}",
        text
    );

    process_text_with_openai(config, prompt).await
}

// Summarize text - support both GET and POST
pub async fn summarize(
    State(config): State<Arc<Config>>,
    text_param: Option<Query<TextRequest>>,
    text_json: Option<Json<TextRequest>>,
) -> Result<impl IntoResponse, AppError> {
    // Extract text from either query parameters or JSON body
    let text = if let Some(query) = text_param {
        query.text.clone()
    } else if let Some(json) = text_json {
        json.text.clone()
    } else {
        return Err(AppError::BadRequest("Text is required".to_string()));
    };

    let prompt = format!("Summarize the following text concisely:\n\n{}", text);

    process_text_with_openai(config, prompt).await
}

// Translate text - support both GET and POST
pub async fn translate(
    State(config): State<Arc<Config>>,
    translation_param: Option<Query<TranslationRequest>>,
    translation_json: Option<Json<TranslationRequest>>,
) -> Result<impl IntoResponse, AppError> {
    // Extract translation request from either query parameters or JSON body
    let translation_request = if let Some(query) = translation_param {
        TranslationRequest {
            text: query.text.clone(),
            target_language: query.target_language.clone(),
        }
    } else if let Some(json) = translation_json {
        TranslationRequest {
            text: json.text.clone(),
            target_language: json.target_language.clone(),
        }
    } else {
        return Err(AppError::BadRequest(
            "Translation parameters are required".to_string(),
        ));
    };

    let target_language_str = match translation_request.target_language {
        TargetLanguage::English => "English",
        TargetLanguage::Spanish => "Spanish",
    };

    let prompt = format!(
        "Translate the following text to {}:\n\n{}",
        target_language_str, translation_request.text
    );

    process_text_with_openai(config, prompt).await
}

// Common function to process text with OpenAI API and return streaming response
async fn process_text_with_openai(
    config: Arc<Config>,
    prompt: String,
) -> Result<impl IntoResponse, AppError> {
    let client = create_client(&config);

    // Create a message for the chat completion
    let message = ChatCompletionRequestMessageArgs::default()
        .role(Role::User)
        .content(prompt)
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to build message: {}", e)))?;

    // Create a chat completion request
    let request = CreateChatCompletionRequestArgs::default()
        .model(&config.openai.model)
        .messages(vec![message])
        .stream(true)
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to build request: {}", e)))?;

    debug!("Sending request to OpenAI");

    // Create a channel for the stream
    let (tx, rx) = mpsc::channel(100);

    // Spawn a task to handle the stream
    tokio::spawn(async move {
        // Create the stream
        let mut stream = client
            .chat()
            .create_stream(request)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to create stream: {}", e);
                panic!("Failed to create stream: {}", e);
            });

        debug!("Stream created successfully");

        while let Some(response) = stream.next().await {
            match response {
                Ok(response) => {
                    for choice in response.choices {
                        if let Some(content) = choice.delta.content {
                            if !content.is_empty() {
                                if let Err(e) = tx.send(Event::default().data(content)).await {
                                    error!("Failed to send event: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error from OpenAI stream: {}", e);
                    let _ = tx
                        .send(Event::default().data(json!({"error": e.to_string()}).to_string()))
                        .await;
                    break;
                }
            }
        }

        // Send a completion event
        let _ = tx.send(Event::default().event("done").data("")).await;
        debug!("Stream completed");
    });

    // Convert the receiver to a stream
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let stream = stream.map(Ok);

    // Create the SSE response with a keep-alive and wrap it with no-cache headers
    let sse =
        Sse::new(stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)));

    // Return the wrapped SSE response with no-cache headers
    Ok(SseWithNoCacheHeaders(sse))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_create_client() {
        let config = Config::default_test_config();
        let client = create_client(&config);

        // Just verify that we can create a client without errors
        // We can't test the async functionality in a sync test, so we'll just check that the client is created
        // No assertion needed - if client creation fails, it will panic
        let _ = client.chat();
    }

    // TODO: Add more comprehensive tests with mocked OpenAI API responses
    // This would require setting up a mock server to simulate OpenAI API responses
}
