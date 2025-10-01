// State persistence
use crate::models::AgentState;
use std::fs;
use std::path::PathBuf;

pub struct StateStore {
    state_file: PathBuf,
}

impl StateStore {
    pub fn new() -> Result<Self, String> {
        let state_file = Self::get_state_file_path()?;
        
        // Ensure directory exists
        if let Some(parent) = state_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create state directory: {}", e))?;
        }
        
        Ok(Self { state_file })
    }
    
    /// Save state to ~/.glassflow/architect-state.json
    pub fn save_state(&self, state: &AgentState) -> Result<(), String> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        
        fs::write(&self.state_file, json)
            .map_err(|e| format!("Failed to write state file: {}", e))?;
        
        Ok(())
    }
    
    /// Load state from ~/.glassflow/architect-state.json
    pub fn load_state(&self) -> Result<AgentState, String> {
        if !self.state_file.exists() {
            return Ok(AgentState::Idle);
        }
        
        let json = fs::read_to_string(&self.state_file)
            .map_err(|e| format!("Failed to read state file: {}", e))?;
        
        let state: AgentState = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize state: {}", e))?;
        
        Ok(state)
    }
    
    /// Clear saved state
    pub fn clear_state(&self) -> Result<(), String> {
        if self.state_file.exists() {
            fs::remove_file(&self.state_file)
                .map_err(|e| format!("Failed to remove state file: {}", e))?;
        }
        Ok(())
    }
    
    /// Get the state file path
    fn get_state_file_path() -> Result<PathBuf, String> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| "Could not determine home directory".to_string())?;
        
        Ok(home_dir.join(".glassflow").join("architect-state.json"))
    }
    
    /// Check if state file exists
    pub fn has_saved_state(&self) -> bool {
        self.state_file.exists()
    }
}
