use serde::{Deserialize, Serialize};
use super::requirement::RequirementCategory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ambiguity {
    pub category: RequirementCategory,
    pub description: String,
    pub impact: Impact,
    pub suggested_questions: Vec<String>,
}

impl Ambiguity {
    pub fn validate(&self) -> Result<(), String> {
        if self.description.is_empty() {
            return Err("Ambiguity description cannot be empty".to_string());
        }
        if self.suggested_questions.is_empty() || self.suggested_questions.len() > 3 {
            return Err("Must have 1-3 suggested questions".to_string());
        }
        Ok(())
    }
}
