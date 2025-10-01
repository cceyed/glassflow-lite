// Quality Agent IPC Commands
use crate::agents::quality::{Quality, QualityConfig, StrictnessLevel};
use crate::models::{CodeOutput, QualityReport};
use crate::llm::client::create_llm_client;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

pub struct QualityAgentState {
    pub agent: Arc<Mutex<Option<Quality>>>,
}

impl QualityAgentState {
    pub fn new() -> Self {
        Self {
            agent: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn quality_review_code(
    code_output: CodeOutput,
    state: State<'_, QualityAgentState>,
) -> Result<QualityReport, String> {
    // Initialize agent if not already created
    let mut agent_lock = state.agent.lock().await;
    
    if agent_lock.is_none() {
        let llm_client = create_llm_client();
        *agent_lock = Some(Quality::new(llm_client));
    }
    
    let agent = agent_lock.as_mut().unwrap();
    
    // Review the code
    agent.review_code(code_output).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn quality_review_code_with_config(
    code_output: CodeOutput,
    strictness: String,
    auto_fix: bool,
    state: State<'_, QualityAgentState>,
) -> Result<QualityReport, String> {
    // Parse strictness level
    let strictness_level = match strictness.as_str() {
        "relaxed" => StrictnessLevel::Relaxed,
        "strict" => StrictnessLevel::Strict,
        _ => StrictnessLevel::Standard,
    };
    
    // Create config
    let config = QualityConfig {
        strictness: strictness_level,
        auto_fix_enabled: auto_fix,
        max_auto_fixes: 50,
        confidence_threshold: 70.0,
    };
    
    // Initialize agent with config
    let mut agent_lock = state.agent.lock().await;
    let llm_client = create_llm_client();
    *agent_lock = Some(Quality::with_config(llm_client, config));
    
    let agent = agent_lock.as_mut().unwrap();
    
    // Review the code
    agent.review_code(code_output).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn quality_get_last_report(
    state: State<'_, QualityAgentState>,
) -> Result<Option<String>, String> {
    let agent_lock = state.agent.lock().await;
    
    if agent_lock.is_none() {
        return Ok(None);
    }
    
    // For now, return a status message
    // In a full implementation, we'd store the last report
    Ok(Some("Quality agent ready".to_string()))
}

#[tauri::command]
pub async fn quality_reset(
    state: State<'_, QualityAgentState>,
) -> Result<(), String> {
    let mut agent_lock = state.agent.lock().await;
    *agent_lock = None;
    Ok(())
}
