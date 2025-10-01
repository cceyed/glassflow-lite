// Engineer Agent module
pub mod plan_analysis;
pub mod code_generator;
pub mod quality_checker;
pub mod auto_fixer;
pub mod confidence;
pub mod file_writer;

pub use plan_analysis::PlanAnalyzer;
pub use code_generator::CodeGenerator;
pub use quality_checker::QualityChecker;
pub use auto_fixer::AutoFixer;
pub use confidence::ConfidenceCalculator;
// FileWriter used internally

// T046-T049: Main Engineer Agent implementation
use crate::models::{
    EngineerState, EngineerArchitecturePlan, GeneratedFile, CodeOutput, 
    QualityReport, GenerationMetadata
};
use crate::codegen::ImportResolver;
// Concurrent modules available for future use
use crate::llm::client::LLMClient;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use tokio::time::{timeout, Duration};

pub struct Engineer {
    state: Arc<Mutex<EngineerState>>,
    llm_client: Box<dyn LLMClient>,
}

impl Engineer {
    pub fn new(llm_client: Box<dyn LLMClient>) -> Self {
        Self {
            state: Arc::new(Mutex::new(EngineerState::Idle)),
            llm_client,
        }
    }

    /// T046: Main implementation method
    pub async fn implement_plan(&mut self, plan: EngineerArchitecturePlan) -> Result<CodeOutput> {
        let start_time = Utc::now();
        
        // T047: State transition to ANALYZING_PLAN
        self.transition_to_analyzing(plan.clone())?;
        
        // Validate and analyze plan
        let analyzer = PlanAnalyzer::new(plan.clone());
        analyzer.validate()?;
        
        let generation_order = analyzer.get_generation_order()?;
        
        // T047: State transition to GENERATING_CODE
        self.transition_to_generating(plan.clone())?;
        
        // Generate files
        let mut generated_files = Vec::new();
        let mut timeouts_encountered = 0;
        
        for (level_idx, level_files) in generation_order.iter().enumerate() {
            self.emit_reasoning(&format!(
                "Generating level {} ({} files)",
                level_idx + 1,
                level_files.len()
            ));
            
            // Generate files in this level (can be parallel)
            for template in level_files {
                // T048: Timeout handling (2 minutes per file)
                match timeout(
                    Duration::from_secs(120),
                    self.generate_file_with_retry(template, &plan)
                ).await {
                    Ok(Ok(file)) => {
                        generated_files.push(file);
                        self.update_progress(generated_files.len(), analyzer.estimate_total_lines());
                    }
                    Ok(Err(e)) => {
                        // T049: Error handling
                        return self.handle_generation_error(
                            e.to_string(),
                            Some(template.path.clone()),
                            generated_files,
                        );
                    }
                    Err(_) => {
                        // Timeout - prompt user
                        timeouts_encountered += 1;
                        match self.handle_timeout(&template.path).await? {
                            TimeoutChoice::Continue => {
                                // Retry without timeout
                                match self.generate_file_with_retry(template, &plan).await {
                                    Ok(file) => generated_files.push(file),
                                    Err(e) => {
                                        return self.handle_generation_error(
                                            e.to_string(),
                                            Some(template.path.clone()),
                                            generated_files,
                                        );
                                    }
                                }
                            }
                            TimeoutChoice::Cancel => {
                                return self.handle_generation_error(
                                    "Generation cancelled by user".to_string(),
                                    Some(template.path.clone()),
                                    generated_files,
                                );
                            }
                            TimeoutChoice::Skip => {
                                self.emit_reasoning(&format!("Skipping {}", template.path));
                                continue;
                            }
                        }
                    }
                }
            }
        }
        
        // T047: State transition to REVIEWING
        self.transition_to_reviewing(generated_files.clone())?;
        
        // Quality check and auto-fix
        let import_resolver = ImportResolver::new(&plan);
        let quality_checker = QualityChecker::new(import_resolver);
        
        let mut all_issues = Vec::new();
        for file in &generated_files {
            let check = quality_checker.check_file(file)?;
            all_issues.extend(check.issues);
        }
        
        self.emit_reasoning(&format!("Found {} issues", all_issues.len()));
        
        // Auto-fix
        let mut fixes_applied = Vec::new();
        for file in &mut generated_files {
            let file_issues: Vec<_> = all_issues.iter()
                .filter(|i| i.file == file.path)
                .cloned()
                .collect();
            
            let fixes = AutoFixer::apply_fixes(file, &file_issues)?;
            fixes_applied.extend(fixes);
        }
        
        if !fixes_applied.is_empty() {
            self.emit_reasoning(&format!("Auto-fixed {} issues", fixes_applied.len()));
        }
        
        // Calculate confidence
        let confidence = ConfidenceCalculator::calculate_confidence(
            &generated_files,
            &all_issues,
            &plan,
        );
        
        // Build quality report
        let critical_count = all_issues.iter()
            .filter(|i| matches!(i.severity, crate::models::Severity::Critical))
            .count();
        let quality_report = QualityReport {
            files_reviewed: generated_files.len(),
            total_lines: generated_files.iter().map(|f| f.lines).sum(),
            issues: all_issues.clone(),
            fixes_applied: fixes_applied.clone(),
            confidence: crate::models::QualityConfidenceBreakdown {
                overall: confidence.overall,
                code_quality: 95.0,
                type_safety: 95.0,
                security: 100.0,
                performance: 90.0,
                maintainability: 90.0,
            },
            duration: std::time::Duration::from_secs(0),
            // Legacy fields
            total_files: generated_files.len(),
            files_passed: generated_files.len() - critical_count,
            files_failed: critical_count,
            issues_found: all_issues.len(),
            issues_fixed: fixes_applied.len(),
            critical_issues: all_issues.iter()
                .filter(|i| matches!(i.severity, crate::models::Severity::Critical))
                .cloned()
                .collect(),
        };
        
        // Build metadata
        let metadata = GenerationMetadata {
            started_at: start_time,
            completed_at: Utc::now(),
            duration: (Utc::now() - start_time).to_std().unwrap_or_default(),
            files_generated_concurrently: generation_order.iter()
                .map(|level| level.len())
                .max()
                .unwrap_or(1),
            timeouts_encountered,
        };
        
        // Create output
        let output = CodeOutput::new(
            generated_files,
            confidence,
            quality_report,
            metadata,
        );
        
        // T047: State transition to COMPLETE
        self.transition_to_complete(output.clone())?;
        
        Ok(output)
    }

    async fn generate_file_with_retry(
        &self,
        template: &crate::models::FileTemplate,
        plan: &EngineerArchitecturePlan,
    ) -> Result<GeneratedFile> {
        // Create a new LLM client for this generation
        let llm_client = crate::llm::client::create_llm_client();
        let generator = CodeGenerator::new(llm_client);
        
        // Try up to 2 times
        for attempt in 1..=2 {
            match generator.generate_file(template, plan).await {
                Ok(file) => return Ok(file),
                Err(e) if attempt < 2 => {
                    self.emit_reasoning(&format!(
                        "Generation failed (attempt {}), retrying: {}",
                        attempt, e
                    ));
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
                Err(e) => return Err(e),
            }
        }
        
        unreachable!()
    }

    // T047: State transitions
    fn transition_to_analyzing(&self, plan: EngineerArchitecturePlan) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let new_state = EngineerState::AnalyzingPlan {
            plan,
            start_time: Some(std::time::Instant::now()),
        };
        state.transition(new_state).map_err(|e| anyhow::anyhow!(e))?;
        self.emit_state_change();
        Ok(())
    }

    fn transition_to_generating(&self, plan: EngineerArchitecturePlan) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let new_state = EngineerState::GeneratingCode {
            plan,
            files_completed: Vec::new(),
            current_file: None,
            progress: 0.0,
        };
        state.transition(new_state).map_err(|e| anyhow::anyhow!(e))?;
        self.emit_state_change();
        Ok(())
    }

    fn transition_to_reviewing(&self, files: Vec<GeneratedFile>) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let new_state = EngineerState::Reviewing {
            generated_files: files,
        };
        state.transition(new_state).map_err(|e| anyhow::anyhow!(e))?;
        self.emit_state_change();
        Ok(())
    }

    fn transition_to_complete(&self, output: CodeOutput) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let new_state = EngineerState::Complete {
            output,
            duration: Some(std::time::Duration::from_secs(0)),
        };
        state.transition(new_state).map_err(|e| anyhow::anyhow!(e))?;
        self.emit_state_change();
        Ok(())
    }

    // T049: Error handling
    fn handle_generation_error(
        &self,
        message: String,
        failed_file: Option<String>,
        completed_files: Vec<GeneratedFile>,
    ) -> Result<CodeOutput> {
        let mut state = self.state.lock().unwrap();
        let new_state = EngineerState::Error {
            message: message.clone(),
            failed_file,
            recoverable: true,
        };
        let _ = state.transition(new_state);
        self.emit_state_change();
        
        // Return partial output with completed files only
        Err(anyhow::anyhow!("Generation failed: {}. {} files completed.", message, completed_files.len()))
    }

    // T048: Timeout handling
    async fn handle_timeout(&self, file_path: &str) -> Result<TimeoutChoice> {
        self.emit_timeout_prompt(file_path);
        
        // In a real implementation, this would wait for user input
        // For now, default to Continue
        Ok(TimeoutChoice::Continue)
    }

    fn update_progress(&self, completed: usize, total: usize) {
        let progress = completed as f32 / total as f32;
        self.emit_progress(progress);
    }

    fn emit_state_change(&self) {
        // Emit state change event (would use Tauri events in real implementation)
    }

    fn emit_reasoning(&self, content: &str) {
        // Emit reasoning event
        eprintln!("[Engineer] {}", content);
    }

    fn emit_progress(&self, progress: f32) {
        // Emit progress event
        eprintln!("[Engineer] Progress: {:.1}%", progress * 100.0);
    }

    fn emit_timeout_prompt(&self, file_path: &str) {
        // Emit timeout prompt event
        eprintln!("[Engineer] Timeout: {}", file_path);
    }

    pub fn get_state(&self) -> EngineerState {
        self.state.lock().unwrap().clone()
    }

    pub fn cancel(&self) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.transition(EngineerState::Idle).map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TimeoutChoice {
    Continue,
    Cancel,
    Skip,
}
