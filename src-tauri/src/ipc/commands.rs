// Tauri IPC commands
use std::sync::Mutex;
use crate::models::{AgentState, SpecificationAnalysis};
use crate::llm::client::{LLMClient, OpenRouterClient};
use crate::agents::architect::{analysis, questions, design, confidence};
use crate::persistence::StateStore;
use serde_json::json;
use tauri::Emitter;

pub struct ArchitectState {
    pub state: Mutex<AgentState>,
    pub llm_client: Box<dyn LLMClient>,
    pub analysis: Mutex<Option<SpecificationAnalysis>>,
    pub state_store: StateStore,
}

impl ArchitectState {
    pub fn new() -> Result<Self, String> {
        let state_store = StateStore::new()?;
        
        // Try to load saved state
        let initial_state = state_store.load_state().unwrap_or(AgentState::Idle);
        
        Ok(Self {
            state: Mutex::new(initial_state),
            llm_client: Box::new(OpenRouterClient::new()?),
            analysis: Mutex::new(None),
            state_store,
        })
    }
    
    /// Save current state to disk
    pub fn persist_state(&self) -> Result<(), String> {
        let current_state = self.state.lock().unwrap();
        self.state_store.save_state(&current_state)
    }
}

#[tauri::command]
pub async fn architect_analyze(
    spec: String,
    state: tauri::State<'_, ArchitectState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    if spec.is_empty() {
        return Err("Specification cannot be empty".to_string());
    }
    
    let start_time = std::time::Instant::now();
    
    // Transition to ANALYZING state
    {
        let mut current_state = state.state.lock().unwrap();
        let old_state = current_state.state_name();
        
        let new_state = AgentState::Analyzing {
            spec: spec.clone(),
            start_time: Some(start_time),
        };
        
        current_state.transition(new_state)
            .map_err(|e| format!("State transition failed: {}", e))?;
        
        emit_state_change(&app, old_state, current_state.state_name());
        emit_reasoning(&app, "analysis_start", &format!("Starting analysis of specification"));
    }
    
    // Parse specification using LLM (now async)
    let analysis_result = analysis::parse_specification(&spec, state.llm_client.as_ref()).await;
    
    match analysis_result {
        Ok(spec_analysis) => {
            emit_reasoning(&app, "analysis_complete", &format!(
                "Analysis complete: {} requirements, {} ambiguities",
                spec_analysis.explicit_requirements.len(),
                spec_analysis.ambiguities.len()
            ));
            
            // Store analysis
            *state.analysis.lock().unwrap() = Some(spec_analysis.clone());
            
            // Decide next state based on ambiguities
            if !spec_analysis.ambiguities.is_empty() && spec_analysis.ambiguities.len() > 2 {
                // Transition to QUESTIONING
                transition_to_questioning(&state, app.clone(), spec_analysis).await?;
            } else {
                // Transition to DESIGNING (spec is clear enough)
                transition_to_designing(&state, app.clone(), spec_analysis).await?;
            }
            
            // Persist state after successful transition
            let _ = state.persist_state();
            
            Ok("Analysis complete".to_string())
        }
        Err(e) => {
            // Transition to ERROR
            let mut current_state = state.state.lock().unwrap();
            let old_state = current_state.state_name();
            
            let error_state = AgentState::Error {
                message: e.clone(),
                recoverable: true,
            };
            
            current_state.transition(error_state)
                .map_err(|err| format!("State transition failed: {}", err))?;
            
            emit_state_change(&app, old_state, current_state.state_name());
            emit_error(&app, &e, true);
            
            Err(e)
        }
    }
}

async fn transition_to_questioning(
    state: &tauri::State<'_, ArchitectState>,
    app: tauri::AppHandle,
    spec_analysis: SpecificationAnalysis,
) -> Result<(), String> {
    emit_reasoning(&app, "generating_questions", "Generating clarifying questions");
    
    // Generate questions from ambiguities (now async)
    let mut question_list = questions::generate_questions(
        &spec_analysis.ambiguities,
        state.llm_client.as_ref()
    ).await?;
    
    // Prioritize and combine
    question_list = questions::prioritize_by_impact(question_list);
    question_list = questions::combine_related_questions(question_list);
    
    emit_reasoning(&app, "questions_ready", &format!("Generated {} questions", question_list.len()));
    
    // Transition to QUESTIONING state
    let mut current_state = state.state.lock().unwrap();
    let old_state = current_state.state_name();
    
    let questioning_state = AgentState::Questioning {
        questions: question_list.clone(),
        answers: std::collections::HashMap::new(),
        current_question_index: 0,
    };
    
    current_state.transition(questioning_state)
        .map_err(|e| format!("State transition failed: {}", e))?;
    
    emit_state_change(&app, old_state, current_state.state_name());
    
    // Emit first question
    if let Some(first_question) = question_list.first() {
        let _ = app.emit("architect:question", json!(first_question));
    }
    
    // Persist questioning state
    drop(current_state);
    let _ = state.persist_state();
    
    Ok(())
}

async fn transition_to_designing(
    state: &tauri::State<'_, ArchitectState>,
    app: tauri::AppHandle,
    spec_analysis: SpecificationAnalysis,
) -> Result<(), String> {
    emit_reasoning(&app, "design_start", "Designing architecture");
    
    // Transition to DESIGNING state
    {
        let mut current_state = state.state.lock().unwrap();
        let old_state = current_state.state_name();
        
        let designing_state = AgentState::Designing {
            analysis: spec_analysis.clone(),
            progress: 0.0,
        };
        
        current_state.transition(designing_state)
            .map_err(|e| format!("State transition failed: {}", e))?;
        
        emit_state_change(&app, old_state, current_state.state_name());
    }
    
    // Generate architecture plan (now async)
    let plan_result = design::design_architecture(&spec_analysis, state.llm_client.as_ref()).await;
    
    match plan_result {
        Ok(arch_plan) => {
            emit_reasoning(&app, "design_complete", &format!(
                "Architecture designed: {} components, {} decisions",
                arch_plan.components.len(),
                arch_plan.decisions.len()
            ));
            
            // Calculate confidence
            let conf = confidence::calculate_confidence(&spec_analysis, &arch_plan);
            
            // Transition to COMPLETE
            let mut current_state = state.state.lock().unwrap();
            let old_state = current_state.state_name();
            let duration = std::time::Duration::from_secs(0); // TODO: calculate actual duration
            
            let complete_state = AgentState::Complete {
                plan: arch_plan.clone(),
                duration: Some(duration),
            };
            
            current_state.transition(complete_state)
                .map_err(|e| format!("State transition failed: {}", e))?;
            
            emit_state_change(&app, old_state, current_state.state_name());
            
            // Emit complete event
            let _ = app.emit("architect:complete", json!({
                "plan": arch_plan,
                "confidence": conf,
            }));
            
            // Persist complete state
            drop(current_state);
            let _ = state.persist_state();
            
            Ok(())
        }
        Err(e) => {
            // Transition to ERROR
            let mut current_state = state.state.lock().unwrap();
            let old_state = current_state.state_name();
            
            let error_state = AgentState::Error {
                message: e.clone(),
                recoverable: true,
            };
            
            current_state.transition(error_state)
                .map_err(|err| format!("State transition failed: {}", err))?;
            
            emit_state_change(&app, old_state, current_state.state_name());
            emit_error(&app, &e, true);
            
            Err(e)
        }
    }
}

// Helper functions for emitting events
fn emit_state_change(app: &tauri::AppHandle, from: &str, to: &str) {
    let _ = app.emit("architect:state-changed", json!({
        "from": from,
        "to": to,
    }));
}

fn emit_reasoning(app: &tauri::AppHandle, reasoning_type: &str, content: &str) {
    let _ = app.emit("architect:reasoning", json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "type": reasoning_type,
        "content": content,
    }));
}

fn emit_error(app: &tauri::AppHandle, message: &str, recoverable: bool) {
    let _ = app.emit("architect:error", json!({
        "message": message,
        "recoverable": recoverable,
    }));
}

#[tauri::command]
pub fn architect_get_state(state: tauri::State<ArchitectState>) -> Result<String, String> {
    let current_state = state.state.lock().unwrap();
    Ok(current_state.state_name().to_string())
}

#[tauri::command]
pub fn architect_cancel(
    state: tauri::State<ArchitectState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut current_state = state.state.lock().unwrap();
    let old_state = current_state.state_name();
    
    // Transition to IDLE (cancel is always allowed)
    current_state.transition(AgentState::Idle)
        .map_err(|e| format!("State transition failed: {}", e))?;
    
    emit_state_change(&app, old_state, current_state.state_name());
    emit_reasoning(&app, "cancelled", "Operation cancelled by user");
    
    // Clear stored analysis
    *state.analysis.lock().unwrap() = None;
    
    // Persist idle state
    drop(current_state);
    let _ = state.persist_state();
    
    Ok(())
}

#[tauri::command]
pub async fn architect_answer(
    answer: String,
    state: tauri::State<'_, ArchitectState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Check state and get necessary data
    let (should_transition, spec_analysis_opt) = {
        let mut current_state = state.state.lock().unwrap();
        
        // Must be in QUESTIONING state
        if let AgentState::Questioning { ref mut questions, ref mut answers, ref mut current_question_index } = *current_state {
            if let Some(current_question) = questions.get(*current_question_index) {
                // Store the answer
                answers.insert(current_question.id.clone(), answer.clone());
                
                emit_reasoning(&app, "answer_received", &format!("Answer received for question {}", *current_question_index + 1));
                
                // Move to next question
                *current_question_index += 1;
                
                // Check if we have more questions
                if *current_question_index < questions.len() {
                    // Emit next question
                    if let Some(next_question) = questions.get(*current_question_index) {
                        let _ = app.emit("architect:question", json!(next_question));
                    }
                    (false, None)
                } else {
                    // All questions answered, need to transition to DESIGNING
                    let spec_analysis = state.analysis.lock().unwrap().clone()
                        .ok_or_else(|| "No analysis found".to_string())?;
                    (true, Some(spec_analysis))
                }
            } else {
                return Err("No current question found".to_string());
            }
        } else {
            return Err(format!("Cannot answer: not in questioning state (current: {})", current_state.state_name()));
        }
    }; // Lock is dropped here
    
    // Now we can await without holding the lock
    if should_transition {
        if let Some(spec_analysis) = spec_analysis_opt {
            transition_to_designing(&state, app, spec_analysis).await?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn architect_export_plan(path: String, state: tauri::State<ArchitectState>) -> Result<(), String> {
    let current_state = state.state.lock().unwrap();
    
    // Must be in COMPLETE state
    if let AgentState::Complete { ref plan, .. } = *current_state {
        let json = serde_json::to_string_pretty(plan)
            .map_err(|e| format!("Failed to serialize plan: {}", e))?;
        
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    } else {
        Err(format!("Cannot export: not in complete state (current: {})", current_state.state_name()))
    }
}

#[tauri::command]
pub async fn architect_retry(
    spec: String,
    state: tauri::State<'_, ArchitectState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    {
        let mut current_state = state.state.lock().unwrap();
        
        // Must be in ERROR state
        if !matches!(*current_state, AgentState::Error { .. }) {
            return Err(format!("Cannot retry: not in error state (current: {})", current_state.state_name()));
        }
        
        let old_state = current_state.state_name();
        
        // Transition back to IDLE first
        current_state.transition(AgentState::Idle)
            .map_err(|e| format!("State transition failed: {}", e))?;
        
        emit_state_change(&app, old_state, current_state.state_name());
    } // Lock is dropped here
    
    // Now call analyze with the new spec
    architect_analyze(spec, state, app).await
}
