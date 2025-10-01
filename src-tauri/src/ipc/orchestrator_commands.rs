// Orchestrator IPC Commands
use crate::orchestrator::{Orchestrator, Phase};
use crate::llm::client::create_llm_client;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

pub struct OrchestratorState {
    pub orchestrator: Arc<Mutex<Option<Orchestrator>>>,
}

impl OrchestratorState {
    pub fn new() -> Self {
        Self {
            orchestrator: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineStatus {
    pub current_phase: String,
    pub progress: f32,
    pub is_running: bool,
}

#[derive(Debug, Serialize)]
pub struct PipelineResponse {
    pub message: String,
    pub confidence: f32,
    pub phases_completed: usize,
    pub total_retries: usize,
    pub files: Vec<GeneratedFileResponse>,
}

#[derive(Debug, Serialize)]
pub struct GeneratedFileResponse {
    pub path: String,
    pub content: String,
    pub language: String,
    pub lines: usize,
}

#[tauri::command]
pub async fn orchestrator_run_pipeline(
    prompt: String,
    app: tauri::AppHandle,
    state: State<'_, OrchestratorState>,
) -> Result<PipelineResponse, String> {
    // Initialize orchestrator if not already created
    let mut orch_lock = state.orchestrator.lock().await;
    
    if orch_lock.is_none() {
        let mut orchestrator = Orchestrator::new()
            .with_app_handle(app.clone());
        orchestrator.initialize().await;
        *orch_lock = Some(orchestrator);
    }
    
    let orchestrator = orch_lock.as_mut().unwrap();
    
    // Run the full pipeline
    match orchestrator.run_pipeline(prompt).await {
        Ok(result) => {
            // Convert generated files to response format
            let files: Vec<GeneratedFileResponse> = result.code_output.files.iter().map(|f| {
                GeneratedFileResponse {
                    path: f.path.clone(),
                    content: f.content.clone(),
                    language: format!("{:?}", f.language),
                    lines: f.lines,
                }
            }).collect();
            
            Ok(PipelineResponse {
                message: format!(
                    "Pipeline complete! Confidence: {:.1}%, Phases: {}, Retries: {}",
                    result.overall_confidence,
                    result.phases_completed,
                    result.total_retries
                ),
                confidence: result.overall_confidence,
                phases_completed: result.phases_completed,
                total_retries: result.total_retries,
                files,
            })
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn orchestrator_get_status(
    state: State<'_, OrchestratorState>,
) -> Result<PipelineStatus, String> {
    let orch_lock = state.orchestrator.lock().await;
    
    if let Some(orchestrator) = orch_lock.as_ref() {
        let phase = orchestrator.get_current_phase();
        let phase_name = match phase {
            Phase::Idle => "Idle",
            Phase::Planning => "Planning",
            Phase::Building => "Building",
            Phase::Validating => "Validating",
            Phase::Testing => "Testing",
            Phase::Complete => "Complete",
            Phase::Error(_) => "Error",
        };
        
        Ok(PipelineStatus {
            current_phase: phase_name.to_string(),
            progress: orchestrator.get_overall_progress(),
            is_running: !matches!(phase, Phase::Idle | Phase::Complete | Phase::Error(_)),
        })
    } else {
        Ok(PipelineStatus {
            current_phase: "Not initialized".to_string(),
            progress: 0.0,
            is_running: false,
        })
    }
}

#[tauri::command]
pub async fn orchestrator_reset(
    state: State<'_, OrchestratorState>,
) -> Result<(), String> {
    let mut orch_lock = state.orchestrator.lock().await;
    *orch_lock = None;
    Ok(())
}
