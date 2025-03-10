use std::convert::Infallible;
use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig as ClientConfig, Client};
use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::Json;
use futures::Stream;
use futures_util::StreamExt;
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::config::Config;
use crate::error::AppError;
use crate::models::{TargetLanguage, TextRequest, TranslationRequest};

// Function to create a client for the OpenAI API
fn create_client(config: &Config) -> Client<ClientConfig> {
    let openai_config = ClientConfig::new()
        .with_api_key(&config.openai.api_key)
        .with_api_base(&config.openai.base_url);

    Client::with_config(openai_config)
}

// Paraphrase text
pub async fn paraphrase(
    State(config): State<Arc<Config>>,
    Json(request): Json<TextRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let prompt = format!(
        "Paraphrase the following text while maintaining its original meaning:\n\n{}",
        request.text
    );

    process_text_with_openai(config, prompt).await
}

// Expand text
pub async fn expand(
    State(config): State<Arc<Config>>,
    Json(request): Json<TextRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let prompt = format!(
        "Expand the following text with more details and explanations:\n\n{}",
        request.text
    );

    process_text_with_openai(config, prompt).await
}

// Summarize text
pub async fn summarize(
    State(config): State<Arc<Config>>,
    Json(request): Json<TextRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let prompt = format!(
        "Summarize the following text concisely while preserving the key points:\n\n{}",
        request.text
    );

    process_text_with_openai(config, prompt).await
}

// Translate text
pub async fn translate(
    State(config): State<Arc<Config>>,
    Json(request): Json<TranslationRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let target_language = match request.target_language {
        TargetLanguage::English => "English",
        TargetLanguage::Spanish => "Spanish",
    };

    let prompt = format!(
        "Translate the following text to {}:\n\n{}",
        target_language, request.text
    );

    process_text_with_openai(config, prompt).await
}

// Common function to process text with OpenAI API and return streaming response
async fn process_text_with_openai(
    config: Arc<Config>,
    prompt: String,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
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

    // Create a channel for streaming the response
    let (tx, rx) = mpsc::channel(100);

    // Spawn a task to handle the streaming response
    tokio::spawn(async move {
        let stream = client.chat().create_stream(request).await;

        if let Err(e) = &stream {
            error!("Failed to create stream: {}", e);
            let _ = tx
                .send(Event::default().data(json!({"error": e.to_string()}).to_string()))
                .await;
            return;
        }

        let mut stream = stream.unwrap();

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

    Ok(Sse::new(stream))
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
