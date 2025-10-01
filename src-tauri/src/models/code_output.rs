// T030: CodeOutput struct - final output bundle from Engineer Agent
use serde::{Deserialize, Serialize};
use super::generated_file::GeneratedFile;
use super::engineer_confidence::ConfidenceBreakdown as EngineerConfidenceBreakdown;
use super::quality_check::QualityIssue;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOutput {
    pub files: Vec<GeneratedFile>,
    pub total_lines: usize,
    pub confidence: EngineerConfidenceBreakdown,
    pub quality_report: QualityReport,
    pub generation_metadata: GenerationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub files_reviewed: usize,
    pub total_lines: usize,
    pub issues: Vec<QualityIssue>,
    pub fixes_applied: Vec<super::quality_check::AppliedFix>,
    pub confidence: super::quality_check::QualityConfidenceBreakdown,
    #[serde(skip)]
    pub duration: std::time::Duration,
    // Legacy fields for compatibility
    pub total_files: usize,
    pub files_passed: usize,
    pub files_failed: usize,
    pub issues_found: usize,
    pub issues_fixed: usize,
    pub critical_issues: Vec<QualityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMetadata {
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    #[serde(skip)]
    pub duration: std::time::Duration,
    pub files_generated_concurrently: usize,
    pub timeouts_encountered: usize,
}

impl CodeOutput {
    pub fn new(
        files: Vec<GeneratedFile>,
        confidence: EngineerConfidenceBreakdown,
        quality_report: QualityReport,
        metadata: GenerationMetadata,
    ) -> Self {
        let total_lines = files.iter().map(|f| f.lines).sum();
        
        Self {
            files,
            total_lines,
            confidence,
            quality_report,
            generation_metadata: metadata,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.files.is_empty() {
            return Err("CodeOutput must contain at least one file".to_string());
        }
        
        if self.total_lines == 0 {
            return Err("Total lines must be greater than 0".to_string());
        }
        
        let calculated_lines: usize = self.files.iter().map(|f| f.lines).sum();
        if calculated_lines != self.total_lines {
            return Err(format!(
                "Total lines mismatch: expected {}, got {}",
                calculated_lines, self.total_lines
            ));
        }
        
        if self.quality_report.total_files != self.files.len() {
            return Err(format!(
                "Quality report file count mismatch: expected {}, got {}",
                self.files.len(),
                self.quality_report.total_files
            ));
        }
        
        self.confidence.validate()?;
        
        Ok(())
    }

    pub fn has_critical_issues(&self) -> bool {
        !self.quality_report.critical_issues.is_empty()
    }

    pub fn success_rate(&self) -> f32 {
        if self.quality_report.total_files == 0 {
            return 0.0;
        }
        
        (self.quality_report.files_passed as f32 / self.quality_report.total_files as f32) * 100.0
    }
}

impl QualityReport {
    pub fn new(total_files: usize) -> Self {
        Self {
            files_reviewed: total_files,
            total_lines: 0,
            issues: Vec::new(),
            fixes_applied: Vec::new(),
            confidence: super::quality_check::QualityConfidenceBreakdown {
                overall: 100.0,
                code_quality: 100.0,
                type_safety: 100.0,
                security: 100.0,
                performance: 100.0,
                maintainability: 100.0,
            },
            duration: std::time::Duration::from_secs(0),
            // Legacy fields
            total_files,
            files_passed: 0,
            files_failed: 0,
            issues_found: 0,
            issues_fixed: 0,
            critical_issues: Vec::new(),
        }
    }

    pub fn all_passed(&self) -> bool {
        self.files_passed == self.total_files && self.critical_issues.is_empty()
    }

    pub fn has_failures(&self) -> bool {
        self.files_failed > 0 || !self.critical_issues.is_empty()
    }
}

impl GenerationMetadata {
    pub fn new(started_at: DateTime<Utc>) -> Self {
        Self {
            started_at,
            completed_at: Utc::now(),
            duration: std::time::Duration::from_secs(0),
            files_generated_concurrently: 0,
            timeouts_encountered: 0,
        }
    }

    pub fn complete(mut self) -> Self {
        self.completed_at = Utc::now();
        self.duration = (self.completed_at - self.started_at)
            .to_std()
            .unwrap_or_default();
        self
    }

    pub fn duration_seconds(&self) -> u64 {
        self.duration.as_secs()
    }
}
