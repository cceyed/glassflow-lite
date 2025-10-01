// Bug Detector - identifies runtime bugs through pattern matching
use crate::models::{Bug, BugSeverity, BugCategory};

pub struct BugDetector;

impl BugDetector {
    pub fn detect_bugs(code: &str, file_path: &str) -> Vec<Bug> {
        let mut bugs = Vec::new();
        
        // Detect null pointer access
        if Self::has_null_pointer_risk(code) {
            bugs.push(Bug {
                id: format!("null_ptr_{}", file_path),
                severity: BugSeverity::High,
                category: BugCategory::NullPointer,
                file: file_path.to_string(),
                line: 0,
                description: "Potential null pointer access".to_string(),
                error_message: "Cannot read property of null/undefined".to_string(),
                code_context: String::new(),
                impact: "Runtime crash".to_string(),
                auto_fixable: true,
                fixed: false,
                suggested_fixes: vec!["Use optional chaining (?.)".to_string()],
            });
        }
        
        // Detect state mutations
        if Self::has_state_mutation(code) {
            bugs.push(Bug {
                id: format!("mutation_{}", file_path),
                severity: BugSeverity::Medium,
                category: BugCategory::StateMutation,
                file: file_path.to_string(),
                line: 0,
                description: "Direct state mutation".to_string(),
                error_message: "State modified directly".to_string(),
                code_context: String::new(),
                impact: "Unpredictable behavior".to_string(),
                auto_fixable: true,
                fixed: false,
                suggested_fixes: vec!["Use immutable update pattern".to_string()],
            });
        }
        
        // Detect unhandled errors
        if Self::has_unhandled_error(code) {
            bugs.push(Bug {
                id: format!("error_{}", file_path),
                severity: BugSeverity::High,
                category: BugCategory::UnhandledError,
                file: file_path.to_string(),
                line: 0,
                description: "Unhandled error in async code".to_string(),
                error_message: "Promise rejection not handled".to_string(),
                code_context: String::new(),
                impact: "Silent failures".to_string(),
                auto_fixable: true,
                fixed: false,
                suggested_fixes: vec!["Add try-catch or .catch()".to_string()],
            });
        }
        
        bugs
    }
    
    fn has_null_pointer_risk(code: &str) -> bool {
        code.contains(".") && !code.contains("?.") && 
        (code.contains("props.") || code.contains("data."))
    }
    
    fn has_state_mutation(code: &str) -> bool {
        code.contains(".push(") || code.contains(".splice(") || code.contains(".sort()")
    }
    
    fn has_unhandled_error(code: &str) -> bool {
        code.contains("await") && !code.contains("try")
    }
}
