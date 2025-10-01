// T034: ImportResolver - validate imports against architecture plan dependencies
use crate::models::{GeneratedFile, EngineerArchitecturePlan};
use std::collections::HashSet;

pub struct ImportResolver {
    plan_dependencies: HashSet<String>,
    generated_files: HashSet<String>,
}

impl ImportResolver {
    pub fn new(plan: &EngineerArchitecturePlan) -> Self {
        let plan_dependencies = plan
            .dependencies
            .iter()
            .map(|d| d.name.clone())
            .collect();
        
        Self {
            plan_dependencies,
            generated_files: HashSet::new(),
        }
    }

    pub fn add_generated_file(&mut self, path: String) {
        self.generated_files.insert(path);
    }

    pub fn validate_imports(&self, file: &GeneratedFile) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        for import in &file.imports {
            let validation_result = self.validate_import(&import.source);
            
            match validation_result {
                ImportValidation::Valid => {},
                ImportValidation::Warning(msg) => {
                    // Warnings don't block, just log
                    eprintln!("Warning: {}", msg);
                },
                ImportValidation::Error(msg) => {
                    errors.push(msg);
                },
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_import(&self, import_path: &str) -> ImportValidation {
        // Relative imports (./  ../)
        if import_path.starts_with("./") || import_path.starts_with("../") {
            return self.validate_relative_import(import_path);
        }
        
        // Node built-ins (always allowed)
        if self.is_node_builtin(import_path) {
            return ImportValidation::Valid;
        }
        
        // External packages
        let package_name = self.extract_package_name(import_path);
        
        if self.plan_dependencies.contains(&package_name) {
            ImportValidation::Valid
        } else {
            ImportValidation::Warning(format!(
                "Package '{}' not in architecture plan dependencies",
                package_name
            ))
        }
    }

    fn validate_relative_import(&self, import_path: &str) -> ImportValidation {
        // Check if the imported file exists in generated files
        // This is a simplified check - full implementation would resolve the path
        let is_generated = self.generated_files.iter().any(|f| {
            import_path.contains(&f.replace("src/", ""))
        });
        
        if is_generated {
            ImportValidation::Valid
        } else {
            ImportValidation::Warning(format!(
                "Relative import '{}' may not resolve to a generated file",
                import_path
            ))
        }
    }

    fn is_node_builtin(&self, import_path: &str) -> bool {
        matches!(
            import_path,
            "fs" | "path" | "http" | "https" | "crypto" | "util" | "stream" | "events" | "buffer" | "url"
        )
    }

    fn extract_package_name(&self, import_path: &str) -> String {
        // Extract package name from import path
        // Examples:
        // "react" -> "react"
        // "react/jsx-runtime" -> "react"
        // "@types/node" -> "@types/node"
        
        if import_path.starts_with('@') {
            // Scoped package
            import_path
                .split('/')
                .take(2)
                .collect::<Vec<_>>()
                .join("/")
        } else {
            // Regular package
            import_path
                .split('/')
                .next()
                .unwrap_or(import_path)
                .to_string()
        }
    }
}

pub enum ImportValidation {
    Valid,
    Warning(String),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ProjectIntent, TechStack, FileStructure, ArchitecturePattern, Dependency};

    fn create_test_plan() -> EngineerArchitecturePlan {
        EngineerArchitecturePlan {
            project_name: "test".to_string(),
            project_intent: ProjectIntent::WebApp,
            tech_stack: TechStack {
                language: "TypeScript".to_string(),
                runtime: "Node.js".to_string(),
                framework: Some("React".to_string()),
                styling: None,
                state_management: None,
                testing: None,
            },
            architecture_pattern: ArchitecturePattern::Atomic,
            file_structure: FileStructure {
                directories: Vec::new(),
                files: Vec::new(),
            },
            component_hierarchy: Vec::new(),
            dependencies: vec![
                Dependency {
                    name: "react".to_string(),
                    version: "^18.0.0".to_string(),
                    dev_only: false,
                },
            ],
            architecture_decisions: Vec::new(),
            confidence_breakdown: None,
        }
    }

    #[test]
    fn test_validate_plan_dependency() {
        let plan = create_test_plan();
        let resolver = ImportResolver::new(&plan);
        
        match resolver.validate_import("react") {
            ImportValidation::Valid => {},
            _ => panic!("Should be valid"),
        }
    }

    #[test]
    fn test_node_builtin() {
        let plan = create_test_plan();
        let resolver = ImportResolver::new(&plan);
        
        assert!(resolver.is_node_builtin("fs"));
        assert!(resolver.is_node_builtin("path"));
        assert!(!resolver.is_node_builtin("react"));
    }

    #[test]
    fn test_extract_package_name() {
        let plan = create_test_plan();
        let resolver = ImportResolver::new(&plan);
        
        assert_eq!(resolver.extract_package_name("react"), "react");
        assert_eq!(resolver.extract_package_name("react/jsx-runtime"), "react");
        assert_eq!(resolver.extract_package_name("@types/node"), "@types/node");
    }
}
