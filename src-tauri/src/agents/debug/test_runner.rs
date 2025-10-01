// Test Runner - executes various test categories
use crate::models::{GeneratedFile, Bug, BugSeverity, TestResult};
use anyhow::Result;

pub struct TestRunner {
    files: Vec<GeneratedFile>,
}

#[derive(Default)]
pub struct BuildTestResult {
    pub passed: bool,
    pub error: String,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    pub fn load_code(&mut self, files: &[GeneratedFile]) -> Result<()> {
        self.files = files.to_vec();
        Ok(())
    }

    pub async fn run_build_tests(&self) -> Result<BuildTestResult> {
        // Simulate build validation
        // In a real implementation, this would:
        // - Run TypeScript compiler
        // - Check for syntax errors
        // - Validate imports
        
        for file in &self.files {
            if file.content.contains("syntax error") {
                return Ok(BuildTestResult {
                    passed: false,
                    error: format!("Syntax error in {}", file.path),
                });
            }
        }
        
        Ok(BuildTestResult {
            passed: true,
            error: String::new(),
        })
    }

    pub async fn run_component_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();
        
        for file in &self.files {
            if file.path.ends_with(".tsx") || file.path.ends_with(".jsx") {
                // Test component rendering
                let result = self.test_component_rendering(file).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }

    async fn test_component_rendering(&self, file: &GeneratedFile) -> Result<TestResult> {
        // Check for common rendering issues
        let mut bug = None;
        
        // Check for null pointer access
        if file.content.contains(".") && !file.content.contains("?.") {
            if file.content.contains("props.") || file.content.contains("todo.") {
                bug = Some(Bug {
                    id: format!("null_access_{}", file.path),
                    severity: BugSeverity::High,
                    category: crate::models::BugCategory::NullPointer,
                    file: file.path.clone(),
                    line: 0,
                    description: "Potential null pointer access".to_string(),
                    error_message: "Cannot read property of null".to_string(),
                    code_context: String::new(),
                    impact: "Component crash".to_string(),
                    auto_fixable: true,
                    fixed: false,
                    suggested_fixes: vec!["Add optional chaining (?.)".to_string()],
                });
            }
        }
        
        Ok(TestResult {
            test_name: format!("Render {}", file.path),
            passed: bug.is_none(),
            bug,
        })
    }

    pub async fn run_state_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();
        
        for file in &self.files {
            if file.path.contains("store") || file.path.contains("state") {
                let result = self.test_state_management(file).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }

    async fn test_state_management(&self, file: &GeneratedFile) -> Result<TestResult> {
        let mut bug = None;
        
        // Check for state mutations
        if file.content.contains(".push(") || file.content.contains(".splice(") {
            bug = Some(Bug {
                id: format!("state_mutation_{}", file.path),
                severity: BugSeverity::Medium,
                category: crate::models::BugCategory::StateMutation,
                file: file.path.clone(),
                line: 0,
                description: "Direct state mutation detected".to_string(),
                error_message: "State should be updated immutably".to_string(),
                code_context: String::new(),
                impact: "Unpredictable state updates".to_string(),
                auto_fixable: true,
                fixed: false,
                suggested_fixes: vec!["Use spread operator [...array, item]".to_string()],
            });
        }
        
        Ok(TestResult {
            test_name: format!("State management {}", file.path),
            passed: bug.is_none(),
            bug,
        })
    }

    pub async fn run_async_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();
        
        for file in &self.files {
            if file.content.contains("async") || file.content.contains("await") {
                let result = self.test_async_operations(file).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }

    async fn test_async_operations(&self, file: &GeneratedFile) -> Result<TestResult> {
        let mut bug = None;
        
        // Check for unhandled promises
        if file.content.contains("await") && !file.content.contains("try") {
            bug = Some(Bug {
                id: format!("unhandled_promise_{}", file.path),
                severity: BugSeverity::High,
                category: crate::models::BugCategory::UnhandledError,
                file: file.path.clone(),
                line: 0,
                description: "Async operation without error handling".to_string(),
                error_message: "Unhandled promise rejection".to_string(),
                code_context: String::new(),
                impact: "Silent failures".to_string(),
                auto_fixable: true,
                fixed: false,
                suggested_fixes: vec!["Add try-catch block".to_string()],
            });
        }
        
        Ok(TestResult {
            test_name: format!("Async operations {}", file.path),
            passed: bug.is_none(),
            bug,
        })
    }
}
