// Quality Checker - performs comprehensive code quality checks
use crate::models::{GeneratedFile, QualityIssue, Severity, CheckCategory};
use crate::agents::quality::StrictnessLevel;
use anyhow::Result;

pub struct QualityChecker {
    strictness: StrictnessLevel,
}

impl QualityChecker {
    pub fn new(strictness: StrictnessLevel) -> Self {
        Self { strictness }
    }

    pub fn check_file(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Run all quality checks
        issues.extend(self.check_syntax(file)?);
        issues.extend(self.check_type_safety(file)?);
        issues.extend(self.check_imports(file)?);
        issues.extend(self.check_code_smells(file)?);
        issues.extend(self.check_security(file)?);
        issues.extend(self.check_performance(file)?);
        issues.extend(self.check_best_practices(file)?);
        issues.extend(self.check_error_handling(file)?);

        Ok(issues)
    }

    fn check_syntax(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Basic syntax validation
        if file.content.contains("syntax error") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Critical,
                category: CheckCategory::Syntax,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Syntax error detected".to_string(),
                impact: "Code will not compile".to_string(),
                suggestion: "Fix syntax error".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        Ok(issues)
    }

    fn check_type_safety(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check for 'any' types
        if file.content.contains(": any") || file.content.contains("<any>") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Medium,
                category: CheckCategory::TypeSafety,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Using 'any' type loses type safety".to_string(),
                impact: "Reduced type checking, potential runtime errors".to_string(),
                suggestion: "Use specific type or generic instead".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        // Check for unsafe type assertions
        if file.content.contains(" as ") && !file.content.contains("// @ts-expect-error") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::High,
                category: CheckCategory::TypeSafety,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Type assertion without validation".to_string(),
                impact: "Runtime error if type assumption is wrong".to_string(),
                suggestion: "Add type guard or runtime validation".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        Ok(issues)
    }

    fn check_imports(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check for unused imports (simplified)
        for import in &file.imports {
            for item in &import.items {
                if !file.content.contains(item) || 
                   file.content.matches(item).count() == 1 {
                    issues.push(QualityIssue {
                        id: uuid::Uuid::new_v4().to_string(),
                        severity: Severity::Low,
                        category: CheckCategory::Imports,
                        file: file.path.clone(),
                        line: None,
                        column: None,
                        description: format!("Unused import '{}'", item),
                        impact: "Unnecessary code, larger bundle size".to_string(),
                        suggestion: "Remove unused import".to_string(),
                        auto_fixable: true,
                        code_context: None,
                    });
                }
            }
        }

        Ok(issues)
    }

    fn check_code_smells(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check file length
        if file.lines > 200 {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Medium,
                category: CheckCategory::CodeSmells,
                file: file.path.clone(),
                line: None,
                column: None,
                description: format!("File is very long ({} lines)", file.lines),
                impact: "Reduced maintainability, harder to navigate".to_string(),
                suggestion: "Consider splitting into smaller files".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        // Check for magic numbers
        let magic_number_patterns = ["86400000", "1000", "3600"];
        for pattern in &magic_number_patterns {
            if file.content.contains(pattern) {
                issues.push(QualityIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Low,
                    category: CheckCategory::CodeSmells,
                    file: file.path.clone(),
                    line: None,
                    column: None,
                    description: format!("Magic number '{}'", pattern),
                    impact: "Unclear meaning, reduced maintainability".to_string(),
                    suggestion: "Extract to named constant".to_string(),
                    auto_fixable: true,
                    code_context: None,
                });
            }
        }

        Ok(issues)
    }

    fn check_security(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check for XSS vulnerabilities
        if file.content.contains("dangerouslySetInnerHTML") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Critical,
                category: CheckCategory::Security,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Potential XSS vulnerability".to_string(),
                impact: "User input rendered without sanitization, code injection risk".to_string(),
                suggestion: "Use textContent or sanitize with DOMPurify".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        // Check for eval usage
        if file.content.contains("eval(") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Critical,
                category: CheckCategory::Security,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Use of eval() detected".to_string(),
                impact: "Code injection vulnerability".to_string(),
                suggestion: "Refactor to avoid dynamic code execution".to_string(),
                auto_fixable: false,
                code_context: None,
            });
        }

        Ok(issues)
    }

    fn check_performance(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check for missing memoization in React components
        if file.path.ends_with(".tsx") && file.content.contains("filter(") {
            if !file.content.contains("useMemo") {
                issues.push(QualityIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Medium,
                    category: CheckCategory::Performance,
                    file: file.path.clone(),
                    line: None,
                    column: None,
                    description: "Array operation without memoization".to_string(),
                    impact: "Re-computes on every render, poor performance".to_string(),
                    suggestion: "Wrap in useMemo".to_string(),
                    auto_fixable: true,
                    code_context: None,
                });
            }
        }

        Ok(issues)
    }

    fn check_best_practices(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check naming conventions
        if file.path.contains("component") && !file.path.chars().next().unwrap().is_uppercase() {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Low,
                category: CheckCategory::BestPractices,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Component filename should be PascalCase".to_string(),
                impact: "Inconsistent naming convention".to_string(),
                suggestion: "Rename to PascalCase".to_string(),
                auto_fixable: true,
                code_context: None,
            });
        }

        Ok(issues)
    }

    fn check_error_handling(&self, file: &GeneratedFile) -> Result<Vec<QualityIssue>> {
        let mut issues = Vec::new();

        // Check for unhandled promises
        if file.content.contains("await ") && !file.content.contains("try {") {
            issues.push(QualityIssue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::High,
                category: CheckCategory::ErrorHandling,
                file: file.path.clone(),
                line: None,
                column: None,
                description: "Async operation without error handling".to_string(),
                impact: "Silent failure, poor UX".to_string(),
                suggestion: "Add try-catch or .catch()".to_string(),
                auto_fixable: true,
                code_context: None,
            });
        }

        Ok(issues)
    }
}
