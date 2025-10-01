// Spec analysis logic
use crate::llm::client::LLMClient;
use crate::models::{
    SpecificationAnalysis, ProjectIntent, Requirement, RequirementCategory, 
    Priority, Source, Ambiguity, Impact
};
use serde_json::Value;

/// Parse specification using LLM to extract requirements and identify ambiguities
pub async fn parse_specification(spec: &str, llm_client: &dyn LLMClient) -> Result<SpecificationAnalysis, String> {
    if spec.trim().is_empty() {
        return Err("Specification cannot be empty".to_string());
    }

    let prompt = format!(
        r#"Analyze this project specification and extract structured information.

Specification:
{}

Respond with a JSON object containing:
{{
  "intent": "WebApp" | "MobileApp" | "DesktopApp" | "API" | "Library" | "CLI" | "Unknown",
  "explicit_requirements": [
    {{"category": "Framework|Language|Styling|StateManagement|Authentication|Database|Deployment|Testing|Other", "content": "requirement text", "priority": "Critical|High|Medium|Low"}}
  ],
  "implicit_requirements": [
    {{"category": "...", "content": "...", "priority": "..."}}
  ],
  "ambiguities": [
    {{"category": "...", "description": "what's unclear", "impact": "High|Medium|Low", "suggested_questions": ["question1", "question2"]}}
  ],
  "technical_keywords": ["keyword1", "keyword2"],
  "missing_critical_info": ["what's missing"]
}}

Be thorough but concise. Identify implicit requirements (e.g., if they mention React, they need Node.js).
Identify ambiguities that need clarification."#,
        spec
    );

    let response = llm_client.send_message(&prompt).await?;
    
    // Parse LLM response
    parse_llm_analysis_response(&response, spec)
}

/// Detect ambiguities in the specification
pub async fn detect_ambiguities(spec: &str, llm_client: &dyn LLMClient) -> Result<Vec<Ambiguity>, String> {
    let prompt = format!(
        r#"Identify ambiguities and unclear aspects in this specification:

{}

Return JSON array of ambiguities:
[
  {{
    "category": "Framework|Language|Styling|StateManagement|Authentication|Database|Deployment|Testing|Other",
    "description": "what's unclear",
    "impact": "High|Medium|Low",
    "suggested_questions": ["question1", "question2"]
  }}
]

Focus on high-impact ambiguities that affect architecture decisions."#,
        spec
    );

    let response = llm_client.send_message(&prompt).await?;
    parse_ambiguities_from_json(&response)
}

/// Categorize requirements by type
pub fn categorize_requirements(requirements: Vec<Requirement>) -> std::collections::HashMap<RequirementCategory, Vec<Requirement>> {
    let mut categorized = std::collections::HashMap::new();
    
    for req in requirements {
        categorized
            .entry(req.category.clone())
            .or_insert_with(Vec::new)
            .push(req);
    }
    
    categorized
}

// Helper functions

fn parse_llm_analysis_response(response: &str, original_spec: &str) -> Result<SpecificationAnalysis, String> {
    // Try to extract JSON from the response
    let json_str = extract_json_from_response(response)?;
    let parsed: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse LLM response as JSON: {}", e))?;

    let intent = parse_project_intent(parsed.get("intent"));
    let explicit_reqs = parse_requirements(parsed.get("explicit_requirements"), Source::Explicit)?;
    let implicit_reqs = parse_requirements(parsed.get("implicit_requirements"), Source::Inferred)?;
    let ambiguities = parse_ambiguities(parsed.get("ambiguities"))?;
    let keywords = parse_string_array(parsed.get("technical_keywords"));
    let missing = parse_string_array(parsed.get("missing_critical_info"));

    // Calculate confidence based on clarity
    let confidence = calculate_spec_clarity(&explicit_reqs, &ambiguities, &missing);

    Ok(SpecificationAnalysis {
        raw_input: original_spec.to_string(),
        intent,
        explicit_requirements: explicit_reqs,
        implicit_requirements: implicit_reqs,
        ambiguities,
        missing_critical_info: missing,
        technical_keywords: keywords,
        confidence,
    })
}

fn extract_json_from_response(response: &str) -> Result<String, String> {
    // Look for JSON in code blocks or raw
    if let Some(start) = response.find("```json") {
        if let Some(end) = response[start..].find("```") {
            let json_start = start + 7; // length of "```json"
            let json_end = start + end;
            return Ok(response[json_start..json_end].trim().to_string());
        }
    }
    
    // Try to find raw JSON object
    if let Some(start) = response.find('{') {
        if let Some(end) = response.rfind('}') {
            return Ok(response[start..=end].to_string());
        }
    }
    
    Err("No JSON found in LLM response".to_string())
}

fn parse_project_intent(value: Option<&Value>) -> ProjectIntent {
    match value.and_then(|v| v.as_str()) {
        Some("WebApp") => ProjectIntent::WebApp,
        Some("MobileApp") => ProjectIntent::MobileApp,
        Some("DesktopApp") => ProjectIntent::DesktopApp,
        Some("API") => ProjectIntent::API,
        Some("Library") => ProjectIntent::Library,
        Some("CLI") => ProjectIntent::CLI,
        _ => ProjectIntent::Unknown,
    }
}

fn parse_requirements(value: Option<&Value>, source: Source) -> Result<Vec<Requirement>, String> {
    let array = match value.and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Ok(Vec::new()),
    };

    let mut requirements = Vec::new();
    for item in array {
        let category = parse_requirement_category(item.get("category"));
        let content = item.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let priority = parse_priority(item.get("priority"));

        if !content.is_empty() {
            requirements.push(Requirement {
                category,
                content,
                priority,
                source: source.clone(),
            });
        }
    }

    Ok(requirements)
}

fn parse_requirement_category(value: Option<&Value>) -> RequirementCategory {
    match value.and_then(|v| v.as_str()) {
        Some("Framework") => RequirementCategory::Framework,
        Some("Language") => RequirementCategory::Language,
        Some("Styling") => RequirementCategory::Styling,
        Some("StateManagement") => RequirementCategory::StateManagement,
        Some("Authentication") => RequirementCategory::Authentication,
        Some("Database") => RequirementCategory::Database,
        Some("Deployment") => RequirementCategory::Deployment,
        Some("Testing") => RequirementCategory::Testing,
        Some(other) => RequirementCategory::Other(other.to_string()),
        None => RequirementCategory::Other("Uncategorized".to_string()),
    }
}

fn parse_priority(value: Option<&Value>) -> Priority {
    match value.and_then(|v| v.as_str()) {
        Some("Critical") => Priority::Critical,
        Some("High") => Priority::High,
        Some("Medium") => Priority::Medium,
        Some("Low") => Priority::Low,
        _ => Priority::Medium,
    }
}

fn parse_ambiguities(value: Option<&Value>) -> Result<Vec<Ambiguity>, String> {
    let array = match value.and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Ok(Vec::new()),
    };

    let mut ambiguities = Vec::new();
    for item in array {
        let category = parse_requirement_category(item.get("category"));
        let description = item.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let impact = parse_impact(item.get("impact"));
        let suggested_questions = parse_string_array(item.get("suggested_questions"));

        if !description.is_empty() && !suggested_questions.is_empty() {
            ambiguities.push(Ambiguity {
                category,
                description,
                impact,
                suggested_questions,
            });
        }
    }

    Ok(ambiguities)
}

fn parse_ambiguities_from_json(response: &str) -> Result<Vec<Ambiguity>, String> {
    let json_str = extract_json_from_response(response)?;
    let parsed: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse ambiguities JSON: {}", e))?;
    
    if let Some(array) = parsed.as_array() {
        let mut ambiguities = Vec::new();
        for item in array {
            let category = parse_requirement_category(item.get("category"));
            let description = item.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let impact = parse_impact(item.get("impact"));
            let suggested_questions = parse_string_array(item.get("suggested_questions"));

            if !description.is_empty() {
                ambiguities.push(Ambiguity {
                    category,
                    description,
                    impact,
                    suggested_questions,
                });
            }
        }
        Ok(ambiguities)
    } else {
        parse_ambiguities(Some(&parsed))
    }
}

fn parse_impact(value: Option<&Value>) -> Impact {
    match value.and_then(|v| v.as_str()) {
        Some("High") => Impact::High,
        Some("Medium") => Impact::Medium,
        Some("Low") => Impact::Low,
        _ => Impact::Medium,
    }
}

fn parse_string_array(value: Option<&Value>) -> Vec<String> {
    match value.and_then(|v| v.as_array()) {
        Some(arr) => arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        None => Vec::new(),
    }
}

fn calculate_spec_clarity(
    explicit_reqs: &[Requirement],
    ambiguities: &[Ambiguity],
    missing: &[String],
) -> f32 {
    let has_reqs = if explicit_reqs.is_empty() { 0.0 } else { 30.0 };
    let ambiguity_penalty = (ambiguities.len() as f32 * 10.0).min(40.0);
    let missing_penalty = (missing.len() as f32 * 5.0).min(20.0);
    let req_bonus = (explicit_reqs.len() as f32 * 5.0).min(30.0);
    
    (has_reqs + req_bonus - ambiguity_penalty - missing_penalty).max(0.0).min(100.0)
}
