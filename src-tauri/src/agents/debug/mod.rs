// Debug Agent Module
// Tests and validates runtime behavior of generated code

pub mod test_runner;
pub mod bug_detector;
pub mod auto_fixer;
pub mod confidence;

pub use test_runner::TestRunner;
// BugDetector used internally
pub use auto_fixer::DebugAutoFixer;
pub use confidence::DebugConfidenceCalculator;

// Main Debug Agent implementation
use crate::llm::client::LLMClient;
use crate::models::{CodeOutput, DebugReport};
use anyhow::Result;
use std::time::Instant;

pub struct Debug {
    llm_client: Box<dyn LLMClient>,
    config: DebugConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DebugConfig {
    pub run_build_tests: bool,
    pub run_component_tests: bool,
    pub run_state_tests: bool,
    pub run_async_tests: bool,
    pub run_edge_case_tests: bool,
    pub auto_fix_enabled: bool,
    pub max_fix_attempts: usize,
    pub confidence_threshold: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            run_build_tests: true,
            run_component_tests: true,
            run_state_tests: true,
            run_async_tests: true,
            run_edge_case_tests: true,
            auto_fix_enabled: true,
            max_fix_attempts: 3,
            confidence_threshold: 70.0,
        }
    }
}

impl Debug {
    pub fn new(llm_client: Box<dyn LLMClient>) -> Self {
        Self {
            llm_client,
            config: DebugConfig::default(),
        }
    }

    pub fn with_config(llm_client: Box<dyn LLMClient>, config: DebugConfig) -> Self {
        Self {
            llm_client,
            config,
        }
    }

    /// Main entry point: test and debug code from Quality agent
    pub async fn test_and_debug(&mut self, code_output: CodeOutput) -> Result<DebugReport> {
        let start_time = Instant::now();
        
        self.emit_reasoning(&format!(
            "Starting debug testing of {} files ({} lines)",
            code_output.files.len(),
            code_output.total_lines
        ));
        
        // Initialize test runner
        let mut test_runner = TestRunner::new();
        
        // Load code into test environment
        test_runner.load_code(&code_output.files)?;
        
        // Run build validation (critical)
        self.emit_reasoning("Running build validation...");
        let build_result = if self.config.run_build_tests {
            test_runner.run_build_tests().await?
        } else {
            Default::default()
        };
        
        if !build_result.passed {
            return Err(anyhow::anyhow!("Build validation failed: {}", build_result.error));
        }
        
        // Run component tests
        self.emit_reasoning("Testing component rendering...");
        let component_results = if self.config.run_component_tests {
            test_runner.run_component_tests().await?
        } else {
            vec![]
        };
        
        // Run state management tests
        self.emit_reasoning("Testing state management...");
        let state_results = if self.config.run_state_tests {
            test_runner.run_state_tests().await?
        } else {
            vec![]
        };
        
        // Run async operation tests
        self.emit_reasoning("Testing async operations...");
        let async_results = if self.config.run_async_tests {
            test_runner.run_async_tests().await?
        } else {
            vec![]
        };
        
        // Collect all bugs
        let mut all_bugs = vec![];
        all_bugs.extend(component_results.iter().filter_map(|r| r.bug.clone()));
        all_bugs.extend(state_results.iter().filter_map(|r| r.bug.clone()));
        all_bugs.extend(async_results.iter().filter_map(|r| r.bug.clone()));
        
        self.emit_reasoning(&format!(
            "Found {} bugs ({} fixable)",
            all_bugs.len(),
            all_bugs.iter().filter(|b| b.auto_fixable).count()
        ));
        
        // Auto-fix bugs if enabled
        let mut fixes_applied = vec![];
        if self.config.auto_fix_enabled && !all_bugs.is_empty() {
            let fixable_bugs: Vec<_> = all_bugs.iter().filter(|b| b.auto_fixable).collect();
            
            if !fixable_bugs.is_empty() {
                self.emit_reasoning(&format!(
                    "Attempting to fix {} bugs",
                    fixable_bugs.len()
                ));
                
                let fixer = DebugAutoFixer::new();
                for bug in fixable_bugs {
                    if let Ok(fix) = fixer.apply_fix(bug).await {
                        fixes_applied.push(fix);
                        self.emit_reasoning(&format!("✓ Fixed: {}", bug.description));
                    }
                }
            }
        }
        
        // Calculate confidence
        let confidence = DebugConfidenceCalculator::calculate(
            &build_result,
            &component_results,
            &state_results,
            &async_results,
            &all_bugs,
        );
        
        // Build report
        let report = DebugReport {
            build_passed: build_result.passed,
            total_tests: component_results.len() + state_results.len() + async_results.len(),
            passed_tests: component_results.iter().filter(|r| r.passed).count()
                + state_results.iter().filter(|r| r.passed).count()
                + async_results.iter().filter(|r| r.passed).count(),
            bugs: all_bugs.clone(),
            fixes_applied,
            confidence,
            duration: start_time.elapsed(),
        };
        
        // Check for critical unfixed bugs
        let critical_unfixed: Vec<_> = all_bugs.iter()
            .filter(|b| b.severity == crate::models::BugSeverity::Critical && !b.fixed)
            .collect();
        
        if !critical_unfixed.is_empty() {
            self.emit_reasoning(&format!(
                "⚠ {} critical bugs remain unfixed",
                critical_unfixed.len()
            ));
            return Err(anyhow::anyhow!(
                "{} critical bugs must be fixed before deployment",
                critical_unfixed.len()
            ));
        }
        
        self.emit_reasoning(&format!(
            "Debug complete - Confidence: {:.1}% ({:.1}s)",
            report.confidence.overall,
            start_time.elapsed().as_secs_f32()
        ));
        
        Ok(report)
    }

    fn emit_reasoning(&self, content: &str) {
        eprintln!("[Debug] {}", content);
    }
}
