// T023: ArchitecturePlan struct (reusing from Architect Agent)
// This is the output from the Architect Agent, input to the Engineer Agent

use serde::{Deserialize, Serialize};
use super::file_template::FileTemplate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchitecturePlan {
    pub project_name: String,
    pub project_intent: ProjectIntent,
    pub tech_stack: TechStack,
    pub architecture_pattern: ArchitecturePattern,
    pub file_structure: FileStructure,
    pub component_hierarchy: Vec<Component>,
    pub dependencies: Vec<Dependency>,
    pub architecture_decisions: Vec<ArchitectureDecision>,
    pub confidence_breakdown: Option<ConfidenceBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
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
pub struct TechStack {
    pub language: String,
    pub runtime: String,
    pub framework: Option<String>,
    pub styling: Option<String>,
    pub state_management: Option<String>,
    pub testing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ArchitecturePattern {
    MVC,
    MVVM,
    Atomic,
    FeatureBased,
    DomainDriven,
    Layered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStructure {
    pub directories: Vec<Directory>,
    pub files: Vec<FileTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub path: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub purpose: String,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub dev_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureDecision {
    pub category: String,
    pub decision: String,
    pub reasoning: String,
    pub alternatives_considered: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceBreakdown {
    pub overall: f32,
    pub spec_clarity: f32,
    pub feasibility: f32,
    pub completeness: f32,
}

impl ArchitecturePlan {
    pub fn validate(&self) -> Result<(), String> {
        if self.project_name.is_empty() {
            return Err("Project name cannot be empty".to_string());
        }
        
        if self.file_structure.files.is_empty() {
            return Err("File structure must contain at least one file".to_string());
        }
        
        // Check for duplicate file paths
        let mut paths = std::collections::HashSet::new();
        for file in &self.file_structure.files {
            if !paths.insert(&file.path) {
                return Err(format!("Duplicate file path: {}", file.path));
            }
        }
        
        Ok(())
    }
}
