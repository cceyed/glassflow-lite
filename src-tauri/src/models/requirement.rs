use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RequirementCategory {
    Framework,
    Language,
    Styling,
    StateManagement,
    Authentication,
    Database,
    Deployment,
    Testing,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Explicit,  // User specified
    Inferred,  // Agent inferred
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub category: RequirementCategory,
    pub content: String,
    pub priority: Priority,
    pub source: Source,
}

impl Requirement {
    pub fn validate(&self) -> Result<(), String> {
        if self.content.is_empty() {
            return Err("Requirement content cannot be empty".to_string());
        }
        Ok(())
    }
}
