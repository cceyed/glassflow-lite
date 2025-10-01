// Question generation
use crate::llm::client::LLMClient;
use crate::models::{Question, QuestionType, QuestionOption, Ambiguity, Impact};
use serde_json::Value;
use uuid::Uuid;

/// Generate clarifying questions from ambiguities
pub async fn generate_questions(
    ambiguities: &[Ambiguity],
    llm_client: &dyn LLMClient,
) -> Result<Vec<Question>, String> {
    if ambiguities.is_empty() {
        return Ok(Vec::new());
    }

    let ambiguities_json = serde_json::to_string_pretty(ambiguities)
        .map_err(|e| format!("Failed to serialize ambiguities: {}", e))?;

    let prompt = format!(
        r#"Generate clarifying questions for these ambiguities:

{}

Return a JSON array of questions:
[
  {{
    "text": "question text",
    "question_type": "SingleChoice|MultipleChoice|YesNo|FreeText|ConfirmationWithDefault",
    "options": [{{"value": "val", "label": "label", "description": "optional"}}],
    "recommended_answer": "optional default",
    "reasoning": "why this question matters",
    "impact": "High|Medium|Low"
  }}
]

Make questions specific, actionable, and focused on architecture decisions.
For choice questions, provide 2-5 realistic options."#,
        ambiguities_json
    );

    let response = llm_client.send_message(&prompt).await?;
    parse_questions_from_json(&response)
}

/// Prioritize questions by impact
pub fn prioritize_by_impact(mut questions: Vec<Question>) -> Vec<Question> {
    questions.sort_by(|a, b| {
        // Sort by impact (High > Medium > Low), then by text
        match (&b.impact, &a.impact) {
            (Impact::High, Impact::High) => a.text.cmp(&b.text),
            (Impact::High, _) => std::cmp::Ordering::Greater,
            (_, Impact::High) => std::cmp::Ordering::Less,
            (Impact::Medium, Impact::Medium) => a.text.cmp(&b.text),
            (Impact::Medium, Impact::Low) => std::cmp::Ordering::Greater,
            (Impact::Low, Impact::Medium) => std::cmp::Ordering::Less,
            (Impact::Low, Impact::Low) => a.text.cmp(&b.text),
        }
    });
    questions
}

/// Combine related questions to reduce count
pub fn combine_related_questions(questions: Vec<Question>) -> Vec<Question> {
    if questions.len() <= 5 {
        return questions;
    }

    // Group by impact level
    let mut high_impact: Vec<Question> = Vec::new();
    let mut medium_impact: Vec<Question> = Vec::new();
    let mut low_impact: Vec<Question> = Vec::new();

    for q in questions {
        match q.impact {
            Impact::High => high_impact.push(q),
            Impact::Medium => medium_impact.push(q),
            Impact::Low => low_impact.push(q),
        }
    }

    // Take top questions from each category
    let mut combined = Vec::new();
    combined.extend(high_impact.into_iter().take(3));
    combined.extend(medium_impact.into_iter().take(2));
    
    // Only add low impact if we have room
    if combined.len() < 5 {
        combined.extend(low_impact.into_iter().take(5 - combined.len()));
    }

    combined
}

// Helper functions

fn parse_questions_from_json(response: &str) -> Result<Vec<Question>, String> {
    let json_str = extract_json_from_response(response)?;
    let parsed: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse questions JSON: {}", e))?;

    let array = match parsed.as_array() {
        Some(arr) => arr,
        None => return Err("Expected JSON array of questions".to_string()),
    };

    let mut questions = Vec::new();
    for item in array {
        if let Some(question) = parse_single_question(item) {
            questions.push(question);
        }
    }

    Ok(questions)
}

fn parse_single_question(value: &Value) -> Option<Question> {
    let text = value.get("text")?.as_str()?.to_string();
    let question_type = parse_question_type(value.get("question_type"));
    let options = parse_options(value.get("options"));
    let recommended_answer = value.get("recommended_answer")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let reasoning = value.get("reasoning")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let impact = parse_impact(value.get("impact"));

    Some(Question {
        id: Uuid::new_v4().to_string(),
        text,
        question_type,
        options,
        recommended_answer,
        reasoning,
        impact,
    })
}

fn parse_question_type(value: Option<&Value>) -> QuestionType {
    match value.and_then(|v| v.as_str()) {
        Some("SingleChoice") => QuestionType::SingleChoice,
        Some("MultipleChoice") => QuestionType::MultipleChoice,
        Some("YesNo") => QuestionType::YesNo,
        Some("FreeText") => QuestionType::FreeText,
        Some("ConfirmationWithDefault") => QuestionType::ConfirmationWithDefault,
        _ => QuestionType::FreeText,
    }
}

fn parse_options(value: Option<&Value>) -> Option<Vec<QuestionOption>> {
    let array = value?.as_array()?;
    let mut options = Vec::new();

    for item in array {
        let value_str = item.get("value")?.as_str()?.to_string();
        let label = item.get("label")?.as_str()?.to_string();
        let description = item.get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        options.push(QuestionOption {
            value: value_str,
            label,
            description,
        });
    }

    if options.is_empty() {
        None
    } else {
        Some(options)
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

fn extract_json_from_response(response: &str) -> Result<String, String> {
    // Look for JSON in code blocks
    if let Some(start) = response.find("```json") {
        if let Some(end) = response[start..].find("```") {
            let json_start = start + 7;
            let json_end = start + end;
            return Ok(response[json_start..json_end].trim().to_string());
        }
    }
    
    // Try to find raw JSON array
    if let Some(start) = response.find('[') {
        if let Some(end) = response.rfind(']') {
            return Ok(response[start..=end].to_string());
        }
    }
    
    Err("No JSON found in LLM response".to_string())
}
