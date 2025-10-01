// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

// Architect Agent modules
mod agents;
mod llm;
mod models;
mod ipc;
mod persistence;

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
    // Initialize Architect state
    let architect_state = ipc::ArchitectState::new()
        .expect("Failed to initialize Architect state");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(architect_state)
        .invoke_handler(tauri::generate_handler![
            process_agent_task,
            get_available_agents,
            ipc::architect_analyze,
            ipc::architect_get_state,
            ipc::architect_cancel,
            ipc::architect_answer,
            ipc::architect_export_plan,
            ipc::architect_retry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
