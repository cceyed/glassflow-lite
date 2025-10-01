// Pipeline - State management for the multi-agent flow

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Idle,
    Planning,      // Architect working
    Building,      // Engineer working
    Validating,    // Quality working
    Testing,       // Debug working
    Complete,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum PipelineDecision {
    Continue,              // Move to next agent
    Retry,                 // Retry current agent
    Rollback(Phase),       // Go back to previous phase
    Fork(Vec<Phase>),      // Run agents in parallel (future)
    Abort,                 // Stop everything
}

pub struct Pipeline {
    current_phase: Phase,
    phase_history: Vec<Phase>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            current_phase: Phase::Idle,
            phase_history: vec![Phase::Idle],
        }
    }
    
    pub fn set_phase(&mut self, phase: Phase) {
        self.phase_history.push(phase.clone());
        self.current_phase = phase;
    }
    
    pub fn current_phase(&self) -> Phase {
        self.current_phase.clone()
    }
    
    pub fn progress(&self) -> f32 {
        match &self.current_phase {
            Phase::Idle => 0.0,
            Phase::Planning => 20.0,
            Phase::Building => 40.0,
            Phase::Validating => 60.0,
            Phase::Testing => 80.0,
            Phase::Complete => 100.0,
            Phase::Error(_) => {
                // Return progress of last successful phase
                self.phase_history.iter()
                    .rev()
                    .find(|p| !matches!(p, Phase::Error(_)))
                    .map(|p| match p {
                        Phase::Idle => 0.0,
                        Phase::Planning => 20.0,
                        Phase::Building => 40.0,
                        Phase::Validating => 60.0,
                        Phase::Testing => 80.0,
                        Phase::Complete => 100.0,
                        _ => 0.0,
                    })
                    .unwrap_or(0.0)
            }
        }
    }
    
    pub fn can_run_parallel(&self) -> bool {
        // Quality and Debug could potentially run in parallel
        // For now, keep it simple and sequential
        false
    }
    
    pub fn phase_name(&self) -> &str {
        match &self.current_phase {
            Phase::Idle => "Idle",
            Phase::Planning => "Planning",
            Phase::Building => "Building",
            Phase::Validating => "Validating",
            Phase::Testing => "Testing",
            Phase::Complete => "Complete",
            Phase::Error(_) => "Error",
        }
    }
    
    pub fn reset(&mut self) {
        self.current_phase = Phase::Idle;
        self.phase_history.clear();
        self.phase_history.push(Phase::Idle);
    }
}
