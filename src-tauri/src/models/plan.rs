use serde::{Deserialize, Serialize};
use super::{ConfidenceBreakdown, analysis::ProjectIntent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchitecturePattern {
    MVC,
    MVVM,
    Atomic,
    FeatureBased,
    DomainDriven,
    Layered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStack {
    pub framework: String,
    pub language: String,
    pub runtime: Option<String>,
    pub bundler: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub purpose: String,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureDecision {
    pub id: String,
    pub category: String,
    pub decision: String,
    pub reasoning: String,
    pub alternatives_considered: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturePlan {
    pub project_name: String,
    pub project_type: ProjectIntent,
    pub tech_stack: TechStack,
    pub architecture_pattern: ArchitecturePattern,
    pub components: Vec<Component>,
    pub decisions: Vec<ArchitectureDecision>,
    pub confidence: ConfidenceBreakdown,
}

impl Default for ArchitecturePlan {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            project_type: ProjectIntent::Unknown,
            tech_stack: TechStack {
                framework: String::new(),
                language: String::new(),
                runtime: None,
                bundler: None,
            },
            architecture_pattern: ArchitecturePattern::FeatureBased,
            components: Vec::new(),
            decisions: Vec::new(),
            confidence: ConfidenceBreakdown::default(),
        }
    }
}

impl ArchitecturePlan {
    pub fn validate(&self) -> Result<(), String> {
        if self.project_name.is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        if self.tech_stack.framework.is_empty() {
            return Err("Framework must be specified".to_string());
        }
        if self.tech_stack.language.is_empty() {
            return Err("Language must be specified".to_string());
        }
        if self.components.is_empty() {
            return Err("Must have at least one component".to_string());
        }
        if self.decisions.is_empty() {
            return Err("Must have at least one architecture decision".to_string());
        }
        Ok(())
    }
}
