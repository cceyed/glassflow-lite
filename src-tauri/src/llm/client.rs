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
            .map_err(|_| "OPENROUTER_API_KEY environment variable not set. Please create a .env file with your API key.".to_string())?;
        
        let model = std::env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "x-ai/grok-4-fast:free".to_string());
        
        // Validate API key format
        if !api_key.starts_with("sk-or-v1-") {
            return Err("Invalid OpenRouter API key format. Key should start with 'sk-or-v1-'".to_string());
        }
        
        Ok(Self {
            api_key,
            model,
            client: reqwest::Client::new(),
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
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            return Err(format!(
                "LLM API error ({}): {}. Check your API key and credits at https://openrouter.ai/",
                status,
                error_body
            ));
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

/// Helper function to create a boxed LLM client
pub fn create_llm_client() -> Box<dyn LLMClient> {
    match OpenRouterClient::new() {
        Ok(client) => Box::new(client),
        Err(e) => {
            eprintln!("❌ Failed to create LLM client: {}", e);
            eprintln!("💡 Make sure you have a .env file with OPENROUTER_API_KEY set");
            eprintln!("   Get your API key from: https://openrouter.ai/keys");
            panic!("LLM client initialization failed: {}", e);
        }
    }
}
