// Architecture design
use crate::llm::client::LLMClient;
use crate::models::{
    ArchitecturePlan, ArchitecturePattern, TechStack, Component, 
    ArchitectureDecision, SpecificationAnalysis, ProjectIntent, ConfidenceBreakdown
};
use serde_json::Value;
use uuid::Uuid;

/// Design architecture plan using LLM
pub async fn design_architecture(
    analysis: &SpecificationAnalysis,
    llm_client: &dyn LLMClient,
) -> Result<ArchitecturePlan, String> {
    let analysis_json = serde_json::to_string_pretty(analysis)
        .map_err(|e| format!("Failed to serialize analysis: {}", e))?;

    let prompt = format!(
        r#"Based on this specification analysis, design a complete architecture plan:

{}

Return a JSON object:
{{
  "project_name": "name",
  "project_type": "WebApp|MobileApp|DesktopApp|API|Library|CLI|Unknown",
  "tech_stack": {{
    "framework": "React|Vue|Angular|Next.js|etc",
    "language": "TypeScript|JavaScript|Python|etc",
    "runtime": "Node.js|Deno|etc",
    "bundler": "Vite|Webpack|etc"
  }},
  "architecture_pattern": "MVC|MVVM|Atomic|FeatureBased|DomainDriven|Layered",
  "components": [
    {{"name": "ComponentName", "purpose": "what it does", "file_path": "src/path/to/file.tsx"}}
  ],
  "decisions": [
    {{
      "category": "Framework|State|Styling|etc",
      "decision": "what was decided",
      "reasoning": "why",
      "alternatives_considered": ["alt1", "alt2"],
      "confidence": 0.85
    }}
  ]
}}

Be specific and practical. Include realistic file paths and component names."#,
        analysis_json
    );

    let response = llm_client.send_message(&prompt).await?;
    parse_architecture_plan(&response, analysis)
}

// Helper functions

fn parse_architecture_plan(
    response: &str,
    analysis: &SpecificationAnalysis,
) -> Result<ArchitecturePlan, String> {
    let json_str = extract_json_from_response(response)?;
    let parsed: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse architecture JSON: {}", e))?;

    let project_name = parsed.get("project_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Untitled Project")
        .to_string();

    let project_type = parse_project_intent(parsed.get("project_type"));
    let tech_stack = parse_tech_stack(parsed.get("tech_stack"))?;
    let architecture_pattern = parse_architecture_pattern(parsed.get("architecture_pattern"));
    let components = parse_components(parsed.get("components"))?;
    let decisions = parse_decisions(parsed.get("decisions"))?;

    // Calculate confidence based on completeness
    let confidence = calculate_plan_confidence(&components, &decisions, analysis);

    // Generate file structure from components
    let file_structure = generate_file_structure(&components, &tech_stack);
    
    // Generate dependencies based on tech stack
    let dependencies = generate_dependencies(&tech_stack);

    Ok(ArchitecturePlan {
        project_name,
        project_type,
        tech_stack,
        architecture_pattern,
        components,
        decisions,
        confidence,
        file_structure,
        dependencies,
    })
}

fn parse_tech_stack(value: Option<&Value>) -> Result<TechStack, String> {
    let obj = value.ok_or("Missing tech_stack")?;
    
    Ok(TechStack {
        framework: obj.get("framework")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        language: obj.get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string(),
        runtime: obj.get("runtime")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        bundler: obj.get("bundler")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    })
}

fn parse_architecture_pattern(value: Option<&Value>) -> ArchitecturePattern {
    match value.and_then(|v| v.as_str()) {
        Some("MVC") => ArchitecturePattern::MVC,
        Some("MVVM") => ArchitecturePattern::MVVM,
        Some("Atomic") => ArchitecturePattern::Atomic,
        Some("FeatureBased") => ArchitecturePattern::FeatureBased,
        Some("DomainDriven") => ArchitecturePattern::DomainDriven,
        Some("Layered") => ArchitecturePattern::Layered,
        _ => ArchitecturePattern::FeatureBased,
    }
}

fn parse_components(value: Option<&Value>) -> Result<Vec<Component>, String> {
    let array = match value.and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Ok(Vec::new()),
    };

    let mut components = Vec::new();
    for item in array {
        let name = item.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let purpose = item.get("purpose")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let file_path = item.get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if !name.is_empty() {
            components.push(Component {
                name,
                purpose,
                file_path,
            });
        }
    }

    Ok(components)
}

fn parse_decisions(value: Option<&Value>) -> Result<Vec<ArchitectureDecision>, String> {
    let array = match value.and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Ok(Vec::new()),
    };

    let mut decisions = Vec::new();
    for item in array {
        let category = item.get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("General")
            .to_string();
        let decision = item.get("decision")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let reasoning = item.get("reasoning")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let alternatives = parse_string_array(item.get("alternatives_considered"));
        let confidence = item.get("confidence")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5) as f32;

        if !decision.is_empty() {
            decisions.push(ArchitectureDecision {
                id: Uuid::new_v4().to_string(),
                category,
                decision,
                reasoning,
                alternatives_considered: alternatives,
                confidence,
            });
        }
    }

    Ok(decisions)
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

fn parse_string_array(value: Option<&Value>) -> Vec<String> {
    match value.and_then(|v| v.as_array()) {
        Some(arr) => arr.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        None => Vec::new(),
    }
}

fn calculate_plan_confidence(
    components: &[Component],
    decisions: &[ArchitectureDecision],
    analysis: &SpecificationAnalysis,
) -> ConfidenceBreakdown {
    // Calculate sub-scores
    let spec_clarity = analysis.confidence / 100.0;
    
    let technical_feasibility = if !components.is_empty() && !decisions.is_empty() {
        0.8
    } else {
        0.5
    };
    
    let architecture_soundness = if decisions.len() >= 3 {
        0.85
    } else {
        0.6
    };
    
    let completeness = if components.len() >= 5 && decisions.len() >= 3 {
        0.9
    } else {
        0.7
    };
    
    let risk_assessment = if analysis.ambiguities.is_empty() {
        0.9
    } else {
        0.7
    };

    ConfidenceBreakdown::calculate(
        spec_clarity,
        technical_feasibility,
        architecture_soundness,
        completeness,
        risk_assessment,
    )
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
    
    // Try to find raw JSON object
    if let Some(start) = response.find('{') {
        if let Some(end) = response.rfind('}') {
            return Ok(response[start..=end].to_string());
        }
    }
    
    Err("No JSON found in LLM response".to_string())
}

fn generate_file_structure(components: &[Component], tech_stack: &TechStack) -> crate::models::plan::FileStructure {
    use crate::models::{FileTemplate, Language, plan::{Directory, FileStructure}};
    
    // Determine language
    let language = match tech_stack.language.as_str() {
        "TypeScript" => Language::TypeScript,
        "JavaScript" => Language::JavaScript,
        "Python" => Language::Python,
        "Rust" => Language::Rust,
        _ => Language::TypeScript,
    };
    
    // Create directories
    let directories = vec![
        Directory {
            path: "src".to_string(),
            purpose: "Source code".to_string(),
        },
        Directory {
            path: "src/components".to_string(),
            purpose: "React components".to_string(),
        },
    ];
    
    // Create file templates from components
    let files: Vec<FileTemplate> = components.iter().map(|c| {
        FileTemplate {
            path: c.file_path.clone(),
            purpose: c.purpose.clone(),
            language: language.clone(),
            dependencies: vec![],
            generation_hints: Some(vec![format!("Export: {}", c.name)]),
            estimated_lines: 50,
        }
    }).collect();
    
    FileStructure {
        directories,
        files,
    }
}

fn generate_dependencies(tech_stack: &TechStack) -> Vec<crate::models::plan::Dependency> {
    use crate::models::plan::Dependency;
    
    let mut deps = Vec::new();
    
    // Add framework dependency
    if !tech_stack.framework.is_empty() {
        deps.push(Dependency {
            name: tech_stack.framework.to_lowercase(),
            version: "latest".to_string(),
            dev_only: false,
        });
    }
    
    // Add common dependencies based on framework
    if tech_stack.framework.to_lowercase().contains("react") {
        deps.push(Dependency {
            name: "react".to_string(),
            version: "^18.0.0".to_string(),
            dev_only: false,
        });
        deps.push(Dependency {
            name: "react-dom".to_string(),
            version: "^18.0.0".to_string(),
            dev_only: false,
        });
    }
    
    // Add TypeScript if needed
    if tech_stack.language == "TypeScript" {
        deps.push(Dependency {
            name: "typescript".to_string(),
            version: "^5.0.0".to_string(),
            dev_only: true,
        });
    }
    
    deps
}
