// T032: CodeGenerator - generate code from FileTemplate using LLM with streaming
use crate::llm::client::LLMClient;
use crate::models::{FileTemplate, GeneratedFile, EngineerArchitecturePlan};
use anyhow::Result;

pub struct CodeGenerator {
    llm_client: Box<dyn LLMClient>,
}

impl CodeGenerator {
    pub fn new(llm_client: Box<dyn LLMClient>) -> Self {
        Self { llm_client }
    }

    pub async fn generate_file(
        &self,
        template: &FileTemplate,
        plan: &EngineerArchitecturePlan,
    ) -> Result<GeneratedFile> {
        let prompt = self.build_generation_prompt(template, plan);
        
        // Generate code using LLM
        let generated_code = self.llm_client.send_message(&prompt).await
            .map_err(|e| anyhow::anyhow!("LLM generation failed: {}", e))?;
        
        // Parse and clean the generated code
        let cleaned_code = self.extract_code_from_response(&generated_code);
        
        // Create GeneratedFile
        let mut file = GeneratedFile::new(
            template.path.clone(),
            cleaned_code,
            template.language.clone(),
        );
        
        // Parse imports, exports, and types (basic implementation)
        file.imports = self.parse_imports(&file.content);
        file.exports = self.parse_exports(&file.content);
        file.types = self.parse_types(&file.content);
        
        // Set initial confidence (will be updated by quality checker)
        file.confidence = 0.8;
        
        Ok(file)
    }

    fn build_generation_prompt(&self, template: &FileTemplate, plan: &EngineerArchitecturePlan) -> String {
        let lang = match template.language {
            crate::models::Language::TypeScript => "TypeScript",
            crate::models::Language::JavaScript => "JavaScript",
            crate::models::Language::Rust => "Rust",
            crate::models::Language::Python => "Python",
            _ => "code",
        };

        format!(
            r#"Generate {lang} code for: {path}

Purpose: {purpose}

Architecture Context:
- Project: {project_name}
- Tech Stack: {tech_stack}
- Pattern: {pattern:?}

Dependencies:
{dependencies}

Style Guide:
- No 'any' types (TypeScript)
- Explicit return types
- JSDoc comments for public APIs
- Optional chaining for null safety
- Try-catch for async operations
- Follow {pattern:?} architecture pattern

Estimated Lines: {estimated_lines}

Generate ONLY the code, no explanations. Ensure:
1. All imports are at the top
2. Proper error handling
3. Type safety
4. Clean, maintainable code
5. Follows best practices

Code:"#,
            lang = lang,
            path = template.path,
            purpose = template.purpose,
            project_name = plan.project_name,
            tech_stack = format!("{} with {}", 
                plan.tech_stack.language,
                plan.tech_stack.framework.as_deref().unwrap_or("no framework")
            ),
            pattern = plan.architecture_pattern,
            dependencies = template.dependencies.join(", "),
            estimated_lines = template.estimated_lines,
        )
    }

    fn extract_code_from_response(&self, response: &str) -> String {
        // Remove markdown code blocks if present
        let code = if let Some(start) = response.find("```") {
            if let Some(end) = response[start + 3..].find("```") {
                let code_start = response[start..].find('\n').map(|i| start + i + 1).unwrap_or(start + 3);
                let code_end = start + 3 + end;
                response[code_start..code_end].trim().to_string()
            } else {
                response.to_string()
            }
        } else {
            response.to_string()
        };

        code.trim().to_string()
    }

    fn parse_imports(&self, content: &str) -> Vec<crate::models::Import> {
        let mut imports = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // TypeScript/JavaScript imports
            if trimmed.starts_with("import ") {
                if let Some(from_pos) = trimmed.find(" from ") {
                    let import_part = &trimmed[7..from_pos].trim();
                    let source_part = &trimmed[from_pos + 6..].trim().trim_matches(|c| c == '\'' || c == '"' || c == ';');
                    
                    let items = if import_part.starts_with('{') && import_part.ends_with('}') {
                        import_part[1..import_part.len()-1]
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect()
                    } else {
                        vec![import_part.to_string()]
                    };
                    
                    imports.push(crate::models::Import::new(
                        source_part.to_string(),
                        items,
                    ));
                }
            }
        }
        
        imports
    }

    fn parse_exports(&self, content: &str) -> Vec<crate::models::Export> {
        let mut exports = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("export default ") {
                let name = trimmed[15..].split_whitespace().next().unwrap_or("default");
                exports.push(crate::models::Export::default_export(name.to_string()));
            } else if trimmed.starts_with("export ") {
                // Named export
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    exports.push(crate::models::Export::named_export(name.to_string()));
                }
            }
        }
        
        exports
    }

    fn parse_types(&self, content: &str) -> Vec<crate::models::TypeDefinition> {
        let mut types = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("interface ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    types.push(crate::models::TypeDefinition {
                        name: name.trim_end_matches('{').trim().to_string(),
                        kind: crate::models::TypeKind::Interface,
                    });
                }
            } else if trimmed.starts_with("type ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    types.push(crate::models::TypeDefinition {
                        name: name.trim_end_matches('=').trim().to_string(),
                        kind: crate::models::TypeKind::Type,
                    });
                }
            } else if trimmed.starts_with("enum ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    types.push(crate::models::TypeDefinition {
                        name: name.trim_end_matches('{').trim().to_string(),
                        kind: crate::models::TypeKind::Enum,
                    });
                }
            } else if trimmed.starts_with("class ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    types.push(crate::models::TypeDefinition {
                        name: name.trim_end_matches('{').trim().to_string(),
                        kind: crate::models::TypeKind::Class,
                    });
                }
            }
        }
        
        types
    }
}
