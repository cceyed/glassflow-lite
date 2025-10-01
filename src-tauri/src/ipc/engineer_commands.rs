// T052-T053: IPC commands for Engineer Agent
use crate::agents::engineer::{Engineer, TimeoutChoice};
use crate::models::{EngineerArchitecturePlan, EngineerState, CodeOutput};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct EngineerAgentState {
    pub agent: Arc<Mutex<Option<Engineer>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub output_directory: String,
    pub max_concurrent: usize,
    pub timeout_seconds: u64,
}

// T052: start_code_generation command
#[tauri::command]
pub async fn start_code_generation(
    plan: EngineerArchitecturePlan,
    config: GenerationConfig,
    state: State<'_, EngineerAgentState>,
) -> Result<String, String> {
    // Check if generation is already in progress
    {
        let agent_lock = state.agent.lock().unwrap();
        if agent_lock.is_some() {
            return Err("Generation already in progress".to_string());
        }
    } // Lock dropped here
    
    // Create new agent instance
    let llm_client = crate::llm::client::create_llm_client();
    let mut agent = Engineer::new(llm_client);
    
    // Start generation (lock is not held during await)
    let result = agent.implement_plan(plan).await;
    
    // Store agent after completion
    let mut agent_lock = state.agent.lock().unwrap();
    match result {
        Ok(output) => {
            *agent_lock = Some(agent);
            Ok(format!("Generation complete: {} files", output.files.len()))
        }
        Err(e) => {
            *agent_lock = Some(agent);
            Err(format!("Generation failed: {}", e))
        }
    }
}

// T052: get_engineer_state command
#[tauri::command]
pub fn get_engineer_state(state: State<EngineerAgentState>) -> Result<EngineerState, String> {
    let agent_lock = state.agent.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        Ok(agent.get_state())
    } else {
        Ok(EngineerState::Idle)
    }
}

// T052: get_generation_progress command
#[tauri::command]
pub fn get_generation_progress(state: State<EngineerAgentState>) -> Result<GenerationProgress, String> {
    let agent_lock = state.agent.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        let agent_state = agent.get_state();
        
        match agent_state {
            EngineerState::GeneratingCode { ref files_completed, progress, .. } => {
                Ok(GenerationProgress {
                    files_completed: files_completed.len(),
                    files_total: 0, // Would need to track this
                    percentage: (progress * 100.0) as u32,
                    current_file: None,
                    elapsed_seconds: 0,
                    estimated_remaining_seconds: None,
                })
            }
            _ => Ok(GenerationProgress::default()),
        }
    } else {
        Ok(GenerationProgress::default())
    }
}

// T052: cancel_generation command
#[tauri::command]
pub fn cancel_generation(state: State<EngineerAgentState>) -> Result<(), String> {
    let mut agent_lock = state.agent.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        agent.cancel().map_err(|e| e.to_string())?;
        *agent_lock = None;
        Ok(())
    } else {
        Err("No generation in progress".to_string())
    }
}

// T052: retry_generation command
#[tauri::command]
pub async fn retry_generation(
    modified_plan: Option<EngineerArchitecturePlan>,
    state: State<'_, EngineerAgentState>,
) -> Result<String, String> {
    let agent_lock = state.agent.lock().unwrap();
    
    // Get the plan from current state or use modified plan
    let plan = if let Some(plan) = modified_plan {
        plan
    } else {
        return Err("No plan provided for retry".to_string());
    };
    
    drop(agent_lock);
    
    // Reset and start new generation
    let config = GenerationConfig {
        output_directory: "./output".to_string(),
        max_concurrent: 3,
        timeout_seconds: 120,
    };
    
    start_code_generation(plan, config, state).await
}

// T052: get_quality_report command
#[tauri::command]
pub fn get_quality_report(state: State<EngineerAgentState>) -> Result<Option<crate::models::QualityReport>, String> {
    let agent_lock = state.agent.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        let agent_state = agent.get_state();
        
        match agent_state {
            EngineerState::Complete { output, .. } => {
                Ok(Some(output.quality_report))
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

// T052: export_generated_code command
#[tauri::command]
pub fn export_generated_code(
    directory: String,
    state: State<EngineerAgentState>,
) -> Result<ExportResult, String> {
    let agent_lock = state.agent.lock().unwrap();
    
    if let Some(agent) = agent_lock.as_ref() {
        let agent_state = agent.get_state();
        
        match agent_state {
            EngineerState::Complete { output, .. } => {
                use crate::agents::engineer::file_writer::FileWriter;
                use std::path::Path;
                
                let writer = FileWriter::new(Path::new(&directory).to_path_buf());
                let paths = writer.write_files(&output.files)
                    .map_err(|e| e.to_string())?;
                
                Ok(ExportResult {
                    files_exported: paths.len(),
                    output_directory: directory,
                })
            }
            _ => Err("No completed generation to export".to_string()),
        }
    } else {
        Err("No generation in progress".to_string())
    }
}

// T052: handle_timeout_prompt command
#[tauri::command]
pub fn handle_timeout_prompt(
    file_path: String,
    choice: String,
) -> Result<(), String> {
    // This would communicate with the running generation task
    // For now, just log the choice
    eprintln!("Timeout choice for {}: {}", file_path, choice);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub files_completed: usize,
    pub files_total: usize,
    pub percentage: u32,
    pub current_file: Option<String>,
    pub elapsed_seconds: u64,
    pub estimated_remaining_seconds: Option<u64>,
}

impl Default for GenerationProgress {
    fn default() -> Self {
        Self {
            files_completed: 0,
            files_total: 0,
            percentage: 0,
            current_file: None,
            elapsed_seconds: 0,
            estimated_remaining_seconds: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub files_exported: usize,
    pub output_directory: String,
}
