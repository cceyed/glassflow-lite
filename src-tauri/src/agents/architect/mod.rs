// Architect Agent Module
// Analyzes specifications, asks clarifying questions, and designs architecture

pub mod analysis;
pub mod questions;
pub mod design;
pub mod confidence;

// Re-export commonly used items (used internally by the agent)
// These are not exposed to external callers but used by the agent's methods

// Main Architect implementation
use crate::llm::client::LLMClient;
use crate::models::{SpecificationAnalysis, ArchitecturePlan};
use anyhow::Result;
use std::collections::HashMap;

pub struct Architect {
    llm_client: Box<dyn LLMClient>,
    config: ArchitectConfig,
}

pub struct ArchitectConfig {
    pub max_questions: usize,
    pub confidence_threshold: f32,
    pub enable_reasoning_stream: bool,
}

impl Default for ArchitectConfig {
    fn default() -> Self {
        Self {
            max_questions: 7,
            confidence_threshold: 85.0,
            enable_reasoning_stream: true,
        }
    }
}

impl Architect {
    pub fn new(llm_client: Box<dyn LLMClient>) -> Self {
        Self {
            llm_client,
            config: ArchitectConfig::default(),
        }
    }

    pub fn with_config(llm_client: Box<dyn LLMClient>, config: ArchitectConfig) -> Self {
        Self {
            llm_client,
            config,
        }
    }

    /// Main entry point: analyze specification and create architecture plan
    pub async fn analyze_and_design(&mut self, spec: String) -> Result<ArchitecturePlan> {
        self.emit_reasoning("Starting specification analysis");
        
        // Parse specification
        let analysis = analysis::parse_specification(&spec, self.llm_client.as_ref()).await
            .map_err(|e| anyhow::anyhow!(e))?;
        
        self.emit_reasoning(&format!(
            "Identified {} explicit requirements, {} ambiguities",
            analysis.explicit_requirements.len(),
            analysis.ambiguities.len()
        ));
        
        // Check if we need to ask questions
        if !analysis.ambiguities.is_empty() && analysis.ambiguities.len() > 2 {
            self.emit_reasoning(&format!(
                "⚠️ Found {} ambiguities, but proceeding with best-effort assumptions in pipeline mode",
                analysis.ambiguities.len()
            ));
            // In pipeline mode, we proceed with reasonable defaults
            // In interactive mode (via IPC), questions would be asked
            // This allows the pipeline to complete without user interaction
        }
        
        // Design architecture
        self.emit_reasoning("Designing system architecture");
        let plan = design::design_architecture(&analysis, self.llm_client.as_ref()).await
            .map_err(|e| anyhow::anyhow!(e))?;
        
        // Calculate confidence
        let confidence = confidence::calculate_confidence(&analysis, &plan);
        
        self.emit_reasoning(&format!(
            "Architecture complete - Confidence: {:.1}%",
            confidence.overall
        ));
        
        Ok(plan)
    }

    /// Process user answers to clarification questions
    pub async fn process_answers(
        &mut self,
        analysis: SpecificationAnalysis,
        answers: HashMap<String, String>,
    ) -> Result<ArchitecturePlan> {
        self.emit_reasoning(&format!("Processing {} answers", answers.len()));
        
        // Update analysis with answers
        let updated_analysis = self.incorporate_answers(analysis, answers).await?;
        
        // Design architecture with complete information
        self.emit_reasoning("Designing architecture with complete information");
        let plan = design::design_architecture(&updated_analysis, self.llm_client.as_ref()).await
            .map_err(|e| anyhow::anyhow!(e))?;
        
        let confidence = confidence::calculate_confidence(&updated_analysis, &plan);
        
        self.emit_reasoning(&format!(
            "Architecture complete - Confidence: {:.1}%",
            confidence.overall
        ));
        
        Ok(plan)
    }

    async fn incorporate_answers(
        &self,
        mut analysis: SpecificationAnalysis,
        answers: HashMap<String, String>,
    ) -> Result<SpecificationAnalysis> {
        // Remove ambiguities that were answered
        let answered_categories: std::collections::HashSet<String> = answers.keys().cloned().collect();
        analysis.ambiguities.retain(|amb| {
            !answered_categories.contains(&format!("{:?}", amb.category))
        });
        
        // Add answers as explicit requirements
        for (category, answer) in answers {
            analysis.explicit_requirements.push(crate::models::Requirement {
                category: crate::models::RequirementCategory::Other(category),
                content: answer,
                priority: crate::models::Priority::High,
                source: crate::models::Source::Explicit,
            });
        }
        
        Ok(analysis)
    }

    fn emit_reasoning(&self, content: &str) {
        if self.config.enable_reasoning_stream {
            eprintln!("[Architect] {}", content);
        }
    }
}
