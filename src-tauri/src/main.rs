// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

// Architect Agent modules
mod agents;
mod llm;
mod models;
mod ipc;
mod persistence;
mod codegen;
mod concurrent;
mod orchestrator;

#[derive(Debug, Serialize, Deserialize)]
struct AgentTask {
    id: String,
    agent_type: String,
    description: String,
    status: String,
    reasoning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentResponse {
    task_id: String,
    result: String,
    reasoning_steps: Vec<String>,
}

// Command to process agent tasks
#[tauri::command]
fn process_agent_task(task: AgentTask) -> Result<AgentResponse, String> {
    // This is where the core agent orchestration logic will live
    // For now, returning a mock response
    Ok(AgentResponse {
        task_id: task.id.clone(),
        result: format!("Processed task: {}", task.description),
        reasoning_steps: vec![
            "Analyzing task requirements".to_string(),
            "Identifying optimal agent strategy".to_string(),
            "Executing task with selected approach".to_string(),
            "Validating results".to_string(),
        ],
    })
}

// Command to get available agents
#[tauri::command]
fn get_available_agents() -> Vec<String> {
    vec![
        "Planner".to_string(),
        "Executor".to_string(),
        "Validator".to_string(),
        "Optimizer".to_string(),
        "Researcher".to_string(),
    ]
}

fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Validate API key is set
    if std::env::var("OPENROUTER_API_KEY").is_err() {
        eprintln!("⚠️  WARNING: OPENROUTER_API_KEY not set!");
        eprintln!("Please create a .env file with your OpenRouter API key.");
        eprintln!("See .env.example for the format.");
    }
    
    // Initialize Architect state
    let architect_state = ipc::ArchitectState::new()
        .expect("Failed to initialize Architect state");
    
    // Initialize Engineer state
    let engineer_state = ipc::EngineerAgentState {
        agent: std::sync::Arc::new(std::sync::Mutex::new(None)),
    };
    
    // Initialize Quality state
    let quality_state = ipc::QualityAgentState::new();
    
    // Initialize Debug state
    let debug_state = ipc::DebugAgentState::new();
    
    // Initialize Orchestrator state
    let orchestrator_state = ipc::OrchestratorState::new();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(architect_state)
        .manage(engineer_state)
        .manage(quality_state)
        .manage(debug_state)
        .manage(orchestrator_state)
        .invoke_handler(tauri::generate_handler![
            process_agent_task,
            get_available_agents,
            ipc::architect_analyze,
            ipc::architect_get_state,
            ipc::architect_cancel,
            ipc::architect_answer,
            ipc::architect_export_plan,
            ipc::architect_retry,
            ipc::start_code_generation,
            ipc::get_engineer_state,
            ipc::get_generation_progress,
            ipc::cancel_generation,
            ipc::retry_generation,
            ipc::get_quality_report,
            ipc::export_generated_code,
            ipc::handle_timeout_prompt,
            ipc::quality_review_code,
            ipc::quality_review_code_with_config,
            ipc::quality_get_last_report,
            ipc::quality_reset,
            ipc::debug_test_code,
            ipc::debug_test_with_config,
            ipc::debug_get_last_report,
            ipc::debug_reset,
            ipc::orchestrator_run_pipeline,
            ipc::orchestrator_get_status,
            ipc::orchestrator_reset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
