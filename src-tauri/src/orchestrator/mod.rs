// Orchestrator Module
// Lightweight coordinator for the multi-agent pipeline

pub mod message_bus;
pub mod pipeline;

pub use message_bus::{MessageBus, Message, MessagePayload};
pub use pipeline::{Pipeline, Phase, PipelineDecision};

use crate::agents::{Architect, Engineer, Quality, Debug};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;
use tauri::Emitter;

/// Orchestrator - The traffic controller for the multi-agent system
/// Keeps it simple: ~500-800 lines, focused on coordination not execution
pub struct Orchestrator {
    // Agent references
    architect: Arc<Mutex<Option<Architect>>>,
    engineer: Arc<Mutex<Option<Engineer>>>,
    quality: Arc<Mutex<Option<Quality>>>,
    debug: Arc<Mutex<Option<Debug>>>,
    
    // Pipeline management
    pipeline: Pipeline,
    
    // Communication
    message_bus: MessageBus,
    
    // Retry tracking
    retry_counts: RetryTracker,
    
    // Tauri app handle for event emission
    app_handle: Option<tauri::AppHandle>,
}

#[derive(Default)]
struct RetryTracker {
    architect_retries: usize,
    engineer_retries: usize,
    quality_retries: usize,
    debug_retries: usize,
}

impl RetryTracker {
    const MAX_RETRIES: usize = 3;
    
    fn can_retry(&self, phase: &Phase) -> bool {
        let count = match phase {
            Phase::Planning => self.architect_retries,
            Phase::Building => self.engineer_retries,
            Phase::Validating => self.quality_retries,
            Phase::Testing => self.debug_retries,
            _ => 0,
        };
        count < Self::MAX_RETRIES
    }
    
    fn increment(&mut self, phase: &Phase) {
        match phase {
            Phase::Planning => self.architect_retries += 1,
            Phase::Building => self.engineer_retries += 1,
            Phase::Validating => self.quality_retries += 1,
            Phase::Testing => self.debug_retries += 1,
            _ => {}
        }
    }
    
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            architect: Arc::new(Mutex::new(None)),
            engineer: Arc::new(Mutex::new(None)),
            quality: Arc::new(Mutex::new(None)),
            debug: Arc::new(Mutex::new(None)),
            pipeline: Pipeline::new(),
            message_bus: MessageBus::new(),
            retry_counts: RetryTracker::default(),
            app_handle: None,
        }
    }
    
    pub fn with_app_handle(mut self, app_handle: tauri::AppHandle) -> Self {
        self.app_handle = Some(app_handle);
        self
    }
    
    /// Initialize all agents (they'll create their own LLM clients as needed)
    pub async fn initialize(&mut self) {
        // Agents will be initialized lazily when first used
        // This avoids the LLM client cloning issue
    }
    
    /// Main entry point: Run the full pipeline
    pub async fn run_pipeline(&mut self, user_prompt: String) -> Result<PipelineResult> {
        self.emit_reasoning("🎬 Starting pipeline orchestration");
        self.retry_counts.reset();
        
        // Phase 1: Planning (Architect)
        self.pipeline.set_phase(Phase::Planning);
        self.emit_reasoning("📋 Phase 1: Architecture Planning");
        
        let architecture_plan = match self.run_architect(user_prompt.clone()).await {
            Ok(plan) => plan,
            Err(e) => return self.handle_error(Phase::Planning, e).await,
        };
        
        // Phase 2: Building (Engineer)
        self.pipeline.set_phase(Phase::Building);
        self.emit_reasoning("⚙️ Phase 2: Code Generation");
        
        let code_output = match self.run_engineer(architecture_plan).await {
            Ok(code) => code,
            Err(e) => return self.handle_error(Phase::Building, e).await,
        };
        
        // Phase 3: Validating (Quality)
        self.pipeline.set_phase(Phase::Validating);
        self.emit_reasoning("✅ Phase 3: Quality Review");
        
        let quality_report = match self.run_quality(code_output.clone()).await {
            Ok(report) => report,
            Err(e) => return self.handle_error(Phase::Validating, e).await,
        };
        
        // Phase 4: Testing (Debug)
        self.pipeline.set_phase(Phase::Testing);
        self.emit_reasoning("🐛 Phase 4: Runtime Testing");
        
        let debug_report = match self.run_debug(code_output.clone()).await {
            Ok(report) => report,
            Err(e) => return self.handle_error(Phase::Testing, e).await,
        };
        
        // Complete
        self.pipeline.set_phase(Phase::Complete);
        self.emit_reasoning("🎉 Pipeline complete!");
        
        let overall_confidence = self.calculate_overall_confidence(&quality_report, &debug_report);
        
        Ok(PipelineResult {
            code_output,
            quality_report,
            debug_report,
            overall_confidence,
            phases_completed: 4,
            total_retries: self.retry_counts.architect_retries 
                + self.retry_counts.engineer_retries
                + self.retry_counts.quality_retries
                + self.retry_counts.debug_retries,
        })
    }
    
    async fn run_architect(&mut self, prompt: String) -> Result<crate::models::ArchitecturePlan> {
        let mut architect = self.architect.lock().await;
        
        // Initialize if needed
        if architect.is_none() {
            let llm_client = crate::llm::client::create_llm_client();
            *architect = Some(Architect::new(llm_client));
        }
        
        let agent = architect.as_mut().unwrap();
        agent.analyze_and_design(prompt).await
    }
    
    async fn run_engineer(&mut self, plan: crate::models::ArchitecturePlan) -> Result<crate::models::CodeOutput> {
        let mut engineer = self.engineer.lock().await;
        
        // Initialize if needed
        if engineer.is_none() {
            let llm_client = crate::llm::client::create_llm_client();
            *engineer = Some(Engineer::new(llm_client));
        }
        
        let agent = engineer.as_mut().unwrap();
        agent.implement_plan(plan).await
    }
    
    async fn run_quality(&mut self, code: crate::models::CodeOutput) -> Result<crate::models::QualityReport> {
        let mut quality = self.quality.lock().await;
        
        // Initialize if needed
        if quality.is_none() {
            let llm_client = crate::llm::client::create_llm_client();
            *quality = Some(Quality::new(llm_client));
        }
        
        let agent = quality.as_mut().unwrap();
        agent.review_code(code).await
    }
    
    async fn run_debug(&mut self, code: crate::models::CodeOutput) -> Result<crate::models::DebugReport> {
        let mut debug = self.debug.lock().await;
        
        // Initialize if needed
        if debug.is_none() {
            let llm_client = crate::llm::client::create_llm_client();
            *debug = Some(Debug::new(llm_client));
        }
        
        let agent = debug.as_mut().unwrap();
        agent.test_and_debug(code).await
    }
    
    async fn handle_error(&mut self, phase: Phase, error: anyhow::Error) -> Result<PipelineResult> {
        let error_msg = error.to_string();
        self.emit_reasoning(&format!("❌ Error in {:?}: {}", phase, error_msg));
        
        // Provide helpful context about the error
        if error_msg.contains("LLM") || error_msg.contains("API") {
            self.emit_reasoning("💡 This appears to be an LLM/API error. Check your API key and connection.");
        } else if error_msg.contains("timeout") {
            self.emit_reasoning("⏱️ Operation timed out. The LLM may be taking longer than expected.");
        } else if error_msg.contains("parse") || error_msg.contains("JSON") {
            self.emit_reasoning("📝 Failed to parse LLM response. The model may have returned invalid data.");
        }
        
        // Decide: retry, rollback, or abort?
        let decision = self.make_decision(&phase, &error);
        
        match decision {
            PipelineDecision::Retry => {
                if self.retry_counts.can_retry(&phase) {
                    self.retry_counts.increment(&phase);
                    self.emit_reasoning(&format!("🔄 Retrying {:?} (attempt {})", phase, 
                        self.get_retry_count(&phase)));
                    // Return error to trigger retry at caller level
                    Err(anyhow::anyhow!("Retry needed: {}", error_msg))
                } else {
                    self.emit_reasoning(&format!("⚠️ Max retries reached for {:?}", phase));
                    Err(anyhow::anyhow!("Max retries exceeded for {:?}. Last error: {}", phase, error_msg))
                }
            }
            PipelineDecision::Rollback(_target_phase) => {
                self.emit_reasoning("⚠️ Rollback not yet implemented. Aborting instead.");
                Err(anyhow::anyhow!("Pipeline failed at {:?}: {}. Rollback not implemented.", phase, error_msg))
            }
            PipelineDecision::Abort => {
                self.emit_reasoning("🛑 Aborting pipeline due to critical error");
                Err(anyhow::anyhow!("Pipeline aborted at {:?}: {}", phase, error_msg))
            }
            _ => Err(anyhow::anyhow!("Unexpected error at {:?}: {}", phase, error_msg)),
        }
    }
    
    fn make_decision(&self, phase: &Phase, error: &anyhow::Error) -> PipelineDecision {
        let error_msg = error.to_string();
        
        // Critical errors abort immediately
        if error_msg.contains("critical") || error_msg.contains("fatal") {
            return PipelineDecision::Abort;
        }
        
        // Clarification needed goes back to planning
        if error_msg.contains("Clarification needed") {
            return PipelineDecision::Rollback(Phase::Planning);
        }
        
        // Otherwise, try to retry
        if self.retry_counts.can_retry(phase) {
            PipelineDecision::Retry
        } else {
            PipelineDecision::Abort
        }
    }
    
    fn get_retry_count(&self, phase: &Phase) -> usize {
        match phase {
            Phase::Planning => self.retry_counts.architect_retries,
            Phase::Building => self.retry_counts.engineer_retries,
            Phase::Validating => self.retry_counts.quality_retries,
            Phase::Testing => self.retry_counts.debug_retries,
            _ => 0,
        }
    }
    
    fn calculate_overall_confidence(
        &self,
        quality: &crate::models::QualityReport,
        debug: &crate::models::DebugReport,
    ) -> f32 {
        // Weighted average: Quality 40%, Debug 60% (runtime is more important)
        (quality.confidence.overall * 0.4) + (debug.confidence.overall * 0.6)
    }
    
    fn emit_reasoning(&self, message: &str) {
        eprintln!("[Orchestrator] {}", message);
        
        // Also emit as Tauri event if app handle is available
        if let Some(app) = &self.app_handle {
            let _ = app.emit("orchestrator:reasoning", json!({
                "content": message,
            }));
        }
    }
    
    pub fn get_current_phase(&self) -> Phase {
        self.pipeline.current_phase()
    }
    
    pub fn get_overall_progress(&self) -> f32 {
        self.pipeline.progress()
    }
}

pub struct PipelineResult {
    pub code_output: crate::models::CodeOutput,
    pub quality_report: crate::models::QualityReport,
    pub debug_report: crate::models::DebugReport,
    pub overall_confidence: f32,
    pub phases_completed: usize,
    pub total_retries: usize,
}
