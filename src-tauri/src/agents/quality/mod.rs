// Quality Agent Module
// Reviews generated code for quality, security, and best practices

pub mod checker;
pub mod fixer;
pub mod confidence;

pub use checker::QualityChecker;
pub use fixer::AutoFixer;
pub use confidence::ConfidenceCalculator;

// Main Quality Agent implementation
use crate::llm::client::LLMClient;
use crate::models::{CodeOutput, QualityReport, Severity};
use anyhow::Result;
use std::time::Instant;

pub struct Quality {
    llm_client: Box<dyn LLMClient>,
    config: QualityConfig,
}

pub struct QualityConfig {
    pub strictness: StrictnessLevel,
    pub auto_fix_enabled: bool,
    pub max_auto_fixes: usize,
    pub confidence_threshold: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum StrictnessLevel {
    Relaxed,   // Fewer checks, more lenient
    Standard,  // Balanced approach
    Strict,    // Maximum checks, zero tolerance
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            strictness: StrictnessLevel::Standard,
            auto_fix_enabled: true,
            max_auto_fixes: 50,
            confidence_threshold: 70.0,
        }
    }
}

impl Quality {
    pub fn new(llm_client: Box<dyn LLMClient>) -> Self {
        Self {
            llm_client,
            config: QualityConfig::default(),
        }
    }

    pub fn with_config(llm_client: Box<dyn LLMClient>, config: QualityConfig) -> Self {
        Self {
            llm_client,
            config,
        }
    }

    /// Main entry point: review code from Engineer
    pub async fn review_code(&mut self, engineer_output: CodeOutput) -> Result<QualityReport> {
        let start_time = Instant::now();
        
        self.emit_reasoning(&format!(
            "Starting quality review of {} files ({} lines)",
            engineer_output.files.len(),
            engineer_output.files.iter().map(|f| f.lines).sum::<usize>()
        ));
        
        // Initialize checker
        let checker = QualityChecker::new(self.config.strictness);
        
        // Review all files
        let mut all_issues = Vec::new();
        for (index, file) in engineer_output.files.iter().enumerate() {
            self.emit_reasoning(&format!(
                "Reviewing {} ({}/{})",
                file.path,
                index + 1,
                engineer_output.files.len()
            ));
            
            let file_issues = checker.check_file(file)?;
            
            if !file_issues.is_empty() {
                self.emit_reasoning(&format!(
                    "Found {} issues in {}",
                    file_issues.len(),
                    file.path
                ));
            }
            
            all_issues.extend(file_issues);
        }
        
        self.emit_reasoning(&format!(
            "Review complete: {} total issues found",
            all_issues.len()
        ));
        
        // Auto-fix if enabled
        let mut fixes_applied = Vec::new();
        if self.config.auto_fix_enabled && !all_issues.is_empty() {
            let fixable_count = all_issues.iter().filter(|i| i.auto_fixable).count();
            
            if fixable_count > 0 {
                self.emit_reasoning(&format!(
                    "Attempting to auto-fix {} issues",
                    fixable_count.min(self.config.max_auto_fixes)
                ));
                
                let fixer = AutoFixer::new();
                fixes_applied = fixer.apply_fixes(&all_issues, &engineer_output.files)?;
                
                if !fixes_applied.is_empty() {
                    self.emit_reasoning(&format!(
                        "Successfully fixed {} issues",
                        fixes_applied.len()
                    ));
                }
            }
        }
        
        // Calculate confidence
        let confidence = ConfidenceCalculator::calculate(&all_issues, &engineer_output.files);
        
        // Build report
        let critical_count = all_issues.iter().filter(|i| matches!(i.severity, Severity::Critical)).count();
        let report = QualityReport {
            files_reviewed: engineer_output.files.len(),
            total_lines: engineer_output.files.iter().map(|f| f.lines).sum(),
            issues: all_issues.clone(),
            fixes_applied: fixes_applied.clone(),
            confidence: confidence.clone(),
            duration: start_time.elapsed(),
            // Legacy fields
            total_files: engineer_output.files.len(),
            files_passed: engineer_output.files.len() - critical_count,
            files_failed: critical_count,
            issues_found: all_issues.len(),
            issues_fixed: fixes_applied.len(),
            critical_issues: all_issues.iter()
                .filter(|i| matches!(i.severity, Severity::Critical))
                .cloned()
                .collect(),
        };
        
        // Check for blocking issues
        let critical_issues: Vec<_> = all_issues.iter()
            .filter(|i| matches!(i.severity, Severity::Critical))
            .cloned()
            .collect();
        
        if !critical_issues.is_empty() {
            self.emit_reasoning(&format!(
                "⚠ {} critical issues found - review blocked",
                critical_issues.len()
            ));
            return Err(anyhow::anyhow!(
                "{} critical issues must be fixed before proceeding",
                critical_issues.len()
            ));
        }
        
        self.emit_reasoning(&format!(
            "Quality review complete - Confidence: {:.1}% ({:.1}s)",
            confidence.overall,
            start_time.elapsed().as_secs_f32()
        ));
        
        Ok(report)
    }

    fn emit_reasoning(&self, content: &str) {
        eprintln!("[Quality] {}", content);
    }
}
