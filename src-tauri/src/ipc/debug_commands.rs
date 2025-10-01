// Debug Agent IPC Commands
use crate::agents::debug::{Debug, DebugConfig};
use crate::models::{CodeOutput, DebugReport};
use crate::llm::client::create_llm_client;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

pub struct DebugAgentState {
    pub agent: Arc<Mutex<Option<Debug>>>,
}

impl DebugAgentState {
    pub fn new() -> Self {
        Self {
            agent: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn debug_test_code(
    code_output: CodeOutput,
    state: State<'_, DebugAgentState>,
) -> Result<DebugReport, String> {
    // Initialize agent if not already created
    let mut agent_lock = state.agent.lock().await;
    
    if agent_lock.is_none() {
        let llm_client = create_llm_client();
        *agent_lock = Some(Debug::new(llm_client));
    }
    
    let agent = agent_lock.as_mut().unwrap();
    
    // Test and debug the code
    agent.test_and_debug(code_output).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn debug_test_with_config(
    code_output: CodeOutput,
    config: DebugConfig,
    state: State<'_, DebugAgentState>,
) -> Result<DebugReport, String> {
    // Initialize agent with config
    let mut agent_lock = state.agent.lock().await;
    let llm_client = create_llm_client();
    *agent_lock = Some(Debug::with_config(llm_client, config));
    
    let agent = agent_lock.as_mut().unwrap();
    
    // Test and debug the code
    agent.test_and_debug(code_output).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn debug_get_last_report(
    state: State<'_, DebugAgentState>,
) -> Result<Option<String>, String> {
    let agent_lock = state.agent.lock().await;
    
    if agent_lock.is_none() {
        return Ok(None);
    }
    
    Ok(Some("Debug agent ready".to_string()))
}

#[tauri::command]
pub async fn debug_reset(
    state: State<'_, DebugAgentState>,
) -> Result<(), String> {
    let mut agent_lock = state.agent.lock().await;
    *agent_lock = None;
    Ok(())
}
