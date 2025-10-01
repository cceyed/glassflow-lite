// LLM client implementation for OpenRouter
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

// Async trait for LLM client
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    async fn send_message(&self, prompt: &str) -> Result<String, String>;
}

pub struct OpenRouterClient {
    api_key: String,
    model: String,
    client: reqwest::Client,  // Changed from blocking::Client
}

impl OpenRouterClient {
    pub fn new() -> Result<Self, String> {
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .unwrap_or_else(|_| "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".to_string());
        let model = std::env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "x-ai/grok-4-fast:free".to_string());
        
        Ok(Self {
            api_key,
            model,
            client: reqwest::Client::new(),  // Changed from blocking::Client
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenRouterClient {
    async fn send_message(&self, prompt: &str) -> Result<String, String> {
        let request = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await  // Added .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let data: OpenRouterResponse = response
            .json()
            .await  // Added .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        data.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| "No response from LLM".to_string())
    }
}
