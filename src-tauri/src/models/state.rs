use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

use super::{SpecificationAnalysis, Question, ArchitecturePlan};

/// Agent state enum representing the 6 possible states
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AgentState {
    Idle,
    Analyzing {
        spec: String,
        #[serde(skip)]
        start_time: Option<Instant>,
    },
    Questioning {
        questions: Vec<Question>,
        answers: HashMap<String, String>,
        current_question_index: usize,
    },
    Designing {
        analysis: SpecificationAnalysis,
        progress: f32,
    },
    Complete {
        plan: ArchitecturePlan,
        #[serde(skip)]
        duration: Option<std::time::Duration>,
    },
    Error {
        message: String,
        recoverable: bool,
    },
}

impl AgentState {
    /// Check if a transition to the target state is valid
    pub fn is_valid_transition(&self, target: &AgentState) -> bool {
        use AgentState::*;
        
        match (self, target) {
            // From IDLE
            (Idle, Analyzing { .. }) => true,
            
            // From ANALYZING
            (Analyzing { .. }, Questioning { .. }) => true,
            (Analyzing { .. }, Designing { .. }) => true,
            (Analyzing { .. }, Error { .. }) => true,
            
            // From QUESTIONING
            (Questioning { .. }, Questioning { .. }) => true, // Advance question
            (Questioning { .. }, Designing { .. }) => true,
            (Questioning { .. }, Error { .. }) => true,
            
            // From DESIGNING
            (Designing { .. }, Complete { .. }) => true,
            (Designing { .. }, Error { .. }) => true,
            
            // From COMPLETE
            (Complete { .. }, Idle) => true,
            
            // From ERROR
            (Error { .. }, Idle) => true,
            (Error { .. }, Analyzing { .. }) => true, // Retry
            
            // Any state can be canceled to IDLE
            (_, Idle) => true,
            
            // Any state can transition to ERROR
            (_, Error { .. }) => true,
            
            // All other transitions are invalid
            _ => false,
        }
    }
    
    /// Perform a state transition with validation
    pub fn transition(&mut self, new_state: AgentState) -> Result<(), String> {
        if !self.is_valid_transition(&new_state) {
            return Err(format!(
                "Invalid state transition from {} to {}",
                self.state_name(),
                new_state.state_name()
            ));
        }
        
        *self = new_state;
        Ok(())
    }
    
    /// Get the state name as a string
    pub fn state_name(&self) -> &'static str {
        match self {
            AgentState::Idle => "idle",
            AgentState::Analyzing { .. } => "analyzing",
            AgentState::Questioning { .. } => "questioning",
            AgentState::Designing { .. } => "designing",
            AgentState::Complete { .. } => "complete",
            AgentState::Error { .. } => "error",
        }
    }
    
    /// Check if the state is terminal (Complete or Error)
    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentState::Complete { .. } | AgentState::Error { .. })
    }
    
    /// Check if the state is active (processing)
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            AgentState::Analyzing { .. } | AgentState::Questioning { .. } | AgentState::Designing { .. }
        )
    }
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let idle = AgentState::Idle;
        let analyzing = AgentState::Analyzing {
            spec: "test".to_string(),
            start_time: None,
        };
        
        assert!(idle.is_valid_transition(&analyzing));
    }

    #[test]
    fn test_invalid_transition_idle_to_complete() {
        let idle = AgentState::Idle;
        let complete = AgentState::Complete {
            plan: ArchitecturePlan::default(),
            duration: None,
        };
        
        assert!(!idle.is_valid_transition(&complete));
    }
}
