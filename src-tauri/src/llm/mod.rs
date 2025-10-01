// LLM Integration Module
// Handles communication with language models for spec parsing and design

pub mod client;
pub mod retry;
pub mod parsers;

pub use client::LLMClient;
pub use retry::retry_with_backoff;
pub use parsers::{extract_tech_keywords, detect_project_intent};
