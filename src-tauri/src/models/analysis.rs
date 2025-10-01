use serde::{Deserialize, Serialize};
use super::{Requirement, Ambiguity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectIntent {
    WebApp,
    MobileApp,
    DesktopApp,
    API,
    Library,
    CLI,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificationAnalysis {
    pub raw_input: String,
    pub intent: ProjectIntent,
    pub explicit_requirements: Vec<Requirement>,
    pub implicit_requirements: Vec<Requirement>,
    pub ambiguities: Vec<Ambiguity>,
    pub missing_critical_info: Vec<String>,
    pub technical_keywords: Vec<String>,
    pub confidence: f32,
}

impl SpecificationAnalysis {
    pub fn validate(&self) -> Result<(), String> {
        if self.raw_input.is_empty() {
            return Err("Raw input cannot be empty".to_string());
        }
        if self.confidence < 0.0 || self.confidence > 100.0 {
            return Err("Confidence must be between 0 and 100".to_string());
        }
        Ok(())
    }
    
    pub fn high_impact_ambiguities(&self) -> Vec<&Ambiguity> {
        self.ambiguities.iter()
            .filter(|a| matches!(a.impact, super::ambiguity::Impact::High))
            .collect()
    }
}
