// T022: EngineerState enum with 6 states
use serde::{Deserialize, Serialize};
use std::time::Instant;
use super::plan::ArchitecturePlan as EngineerArchitecturePlan;
use super::generated_file::GeneratedFile;
use super::code_output::CodeOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineerState {
    Idle,
    AnalyzingPlan {
        plan: EngineerArchitecturePlan,
        #[serde(skip)]
        start_time: Option<Instant>,
    },
    GeneratingCode {
        plan: EngineerArchitecturePlan,
        files_completed: Vec<GeneratedFile>,
        current_file: Option<String>,
        progress: f32, // 0.0-1.0
    },
    Reviewing {
        generated_files: Vec<GeneratedFile>,
    },
    Complete {
        output: CodeOutput,
        #[serde(skip)]
        duration: Option<std::time::Duration>,
    },
    Error {
        message: String,
        failed_file: Option<String>,
        recoverable: bool,
    },
}

impl EngineerState {
    pub fn state_name(&self) -> &str {
        match self {
            EngineerState::Idle => "idle",
            EngineerState::AnalyzingPlan { .. } => "analyzing_plan",
            EngineerState::GeneratingCode { .. } => "generating_code",
            EngineerState::Reviewing { .. } => "reviewing",
            EngineerState::Complete { .. } => "complete",
            EngineerState::Error { .. } => "error",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, EngineerState::Complete { .. } | EngineerState::Error { .. })
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self,
            EngineerState::AnalyzingPlan { .. }
                | EngineerState::GeneratingCode { .. }
                | EngineerState::Reviewing { .. }
        )
    }

    pub fn can_transition_to(&self, next: &EngineerState) -> bool {
        match (self, next) {
            // From Idle
            (EngineerState::Idle, EngineerState::AnalyzingPlan { .. }) => true,
            
            // From AnalyzingPlan
            (EngineerState::AnalyzingPlan { .. }, EngineerState::GeneratingCode { .. }) => true,
            (EngineerState::AnalyzingPlan { .. }, EngineerState::Error { .. }) => true,
            
            // From GeneratingCode
            (EngineerState::GeneratingCode { .. }, EngineerState::Reviewing { .. }) => true,
            (EngineerState::GeneratingCode { .. }, EngineerState::Error { .. }) => true,
            (EngineerState::GeneratingCode { .. }, EngineerState::Idle) => true, // Cancel
            
            // From Reviewing
            (EngineerState::Reviewing { .. }, EngineerState::Complete { .. }) => true,
            (EngineerState::Reviewing { .. }, EngineerState::GeneratingCode { .. }) => true, // Re-generate after fixes
            (EngineerState::Reviewing { .. }, EngineerState::Error { .. }) => true,
            
            // From Complete
            (EngineerState::Complete { .. }, EngineerState::Idle) => true,
            
            // From Error
            (EngineerState::Error { .. }, EngineerState::Idle) => true,
            (EngineerState::Error { .. }, EngineerState::AnalyzingPlan { .. }) => true, // Retry
            
            _ => false,
        }
    }

    pub fn transition(&mut self, next: EngineerState) -> Result<(), String> {
        if !self.can_transition_to(&next) {
            return Err(format!(
                "Invalid state transition from {} to {}",
                self.state_name(),
                next.state_name()
            ));
        }
        *self = next;
        Ok(())
    }
}

impl Default for EngineerState {
    fn default() -> Self {
        EngineerState::Idle
    }
}
