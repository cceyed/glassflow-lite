// T033: TemplateEngine - build generation prompts with architecture context
use crate::models::{FileTemplate, EngineerArchitecturePlan};

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn build_prompt(template: &FileTemplate, plan: &EngineerArchitecturePlan) -> String {
        let mut prompt = String::new();
        
        // Header
        prompt.push_str(&format!("# Code Generation Task\n\n"));
        prompt.push_str(&format!("**File**: {}\n", template.path));
        prompt.push_str(&format!("**Purpose**: {}\n", template.purpose));
        prompt.push_str(&format!("**Language**: {:?}\n\n", template.language));
        
        // Architecture Context
        prompt.push_str("## Architecture Context\n\n");
        prompt.push_str(&format!("- **Project**: {}\n", plan.project_name));
        prompt.push_str(&format!("- **Type**: {:?}\n", plan.project_intent));
        prompt.push_str(&format!("- **Language**: {}\n", plan.tech_stack.language));
        
        if let Some(framework) = &plan.tech_stack.framework {
            prompt.push_str(&format!("- **Framework**: {}\n", framework));
        }
        
        prompt.push_str(&format!("- **Pattern**: {:?}\n\n", plan.architecture_pattern));
        
        // Dependencies
        if !template.dependencies.is_empty() {
            prompt.push_str("## Dependencies\n\n");
            for dep in &template.dependencies {
                prompt.push_str(&format!("- {}\n", dep));
            }
            prompt.push_str("\n");
        }
        
        // Generation Hints
        if let Some(hints) = &template.generation_hints {
            if !hints.is_empty() {
                prompt.push_str("## Generation Hints\n\n");
                for hint in hints {
                    prompt.push_str(&format!("- {}\n", hint));
                }
                prompt.push_str("\n");
            }
        }
        
        // Requirements
        prompt.push_str("## Requirements\n\n");
        prompt.push_str("1. Generate clean, production-ready code\n");
        prompt.push_str("2. Follow TypeScript best practices\n");
        prompt.push_str("3. Include proper error handling\n");
        prompt.push_str("4. Add JSDoc comments for public APIs\n");
        prompt.push_str("5. Use type-safe patterns\n");
        prompt.push_str("6. Implement null safety with optional chaining\n");
        prompt.push_str(&format!("7. Target approximately {} lines\n\n", template.estimated_lines));
        
        prompt.push_str("Generate the code below:\n\n");
        
        prompt
    }
}
