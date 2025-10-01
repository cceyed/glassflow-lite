use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchitectPhase {
    Analyzing,
    Questioning,
    Designing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningType {
    Observation,
    Analysis,
    Decision,
    Question,
    Conclusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningEntry {
    pub timestamp: DateTime<Utc>,
    pub phase: ArchitectPhase,
    #[serde(rename = "type")]
    pub type_: ReasoningType,
    pub content: String,
    pub confidence: Option<f32>,
    pub related_to: Option<String>,
}

impl ReasoningEntry {
    pub fn new(phase: ArchitectPhase, type_: ReasoningType, content: String) -> Self {
        Self {
            timestamp: Utc::now(),
            phase,
            type_,
            content,
            confidence: None,
            related_to: None,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.content.is_empty() {
            return Err("Reasoning content cannot be empty".to_string());
        }
        if let Some(conf) = self.confidence {
            if conf < 0.0 || conf > 100.0 {
                return Err("Confidence must be between 0 and 100".to_string());
            }
        }
        Ok(())
    }
}
