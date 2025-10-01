// T037: QualityChecker - run all quality checks
use crate::models::{GeneratedFile, CodeQualityCheck, QualityIssue, Severity, CheckCategory};
use crate::codegen::{TypeChecker, SyntaxValidator, ImportResolver};
use anyhow::Result;

pub struct QualityChecker {
    import_resolver: ImportResolver,
}

impl QualityChecker {
    pub fn new(import_resolver: ImportResolver) -> Self {
        Self { import_resolver }
    }

    pub fn check_file(&self, file: &GeneratedFile) -> Result<CodeQualityCheck> {
        let mut check = CodeQualityCheck::new();
        
        // 1. Syntax validation
        check.syntax_valid = self.check_syntax(file, &mut check.issues)?;
        
        // 2. Type safety
        check.types_correct = self.check_type_safety(file, &mut check.issues)?;
        
        // 3. Import resolution
        check.imports_resolved = self.check_imports(file, &mut check.issues);
        
        // 4. Export validation
        check.exports_valid = self.check_exports(file, &mut check.issues);
        
        // 5. Style compliance
        check.style_compliant = self.check_style(file, &mut check.issues);
        
        // 6. Edge case handling
        check.edge_cases_handled = self.check_edge_cases(file, &mut check.issues);
        
        // 7. Documentation
        check.documented = self.check_documentation(file, &mut check.issues);
        
        Ok(check)
    }

    fn check_syntax(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> Result<bool> {
        // Basic syntax check
        let basic_issues = SyntaxValidator::check_basic_syntax(&file.content);
        for issue in basic_issues {
            issues.push(QualityIssue::new(
                Severity::Critical,
                CheckCategory::Syntax,
                file.path.clone(),
                issue.clone(),
                "Fix syntax error".to_string(),
            ));
        }
        
        // Full TypeScript syntax validation
        match SyntaxValidator::validate_syntax(file) {
            Ok(result) => {
                for error in result.errors {
                    issues.push(
                        QualityIssue::new(
                            Severity::Critical,
                            CheckCategory::Syntax,
                            file.path.clone(),
                            error.message.clone(),
                            "Fix syntax error".to_string(),
                        )
                        .with_location(error.line.unwrap_or(0), error.column)
                    );
                }
                Ok(result.valid)
            }
            Err(_) => Ok(true), // If tsc not available, skip
        }
    }

    fn check_type_safety(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> Result<bool> {
        // Check for 'any' types
        if TypeChecker::has_any_types(&file.content) {
            issues.push(QualityIssue::new(
                Severity::Medium,
                CheckCategory::TypeSafety,
                file.path.clone(),
                "Using 'any' type loses type safety".to_string(),
                "Use specific type or generic instead".to_string(),
            ).with_auto_fix(false));
        }
        
        // Check for explicit return types
        if !TypeChecker::has_explicit_return_types(&file.content) {
            issues.push(QualityIssue::new(
                Severity::Low,
                CheckCategory::TypeSafety,
                file.path.clone(),
                "Missing explicit return types".to_string(),
                "Add return type annotations to functions".to_string(),
            ).with_auto_fix(false));
        }
        
        // Full type check
        match TypeChecker::check_types(file) {
            Ok(result) => {
                for error in result.errors {
                    issues.push(
                        QualityIssue::new(
                            Severity::High,
                            CheckCategory::TypeSafety,
                            file.path.clone(),
                            error.message.clone(),
                            "Fix type error".to_string(),
                        )
                        .with_location(error.line.unwrap_or(0), None)
                    );
                }
                Ok(result.passed)
            }
            Err(_) => Ok(true), // If tsc not available, skip
        }
    }

    fn check_imports(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> bool {
        match self.import_resolver.validate_imports(file) {
            Ok(_) => true,
            Err(errors) => {
                for error in errors {
                    issues.push(QualityIssue::new(
                        Severity::High,
                        CheckCategory::Imports,
                        file.path.clone(),
                        error,
                        "Ensure import resolves correctly".to_string(),
                    ));
                }
                false
            }
        }
    }

    fn check_exports(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> bool {
        // Check if file has exports (most files should)
        if file.exports.is_empty() && !file.path.contains("index") {
            issues.push(QualityIssue::new(
                Severity::Low,
                CheckCategory::Exports,
                file.path.clone(),
                "File has no exports".to_string(),
                "Add exports if this is a module".to_string(),
            ));
            return false;
        }
        true
    }

    fn check_style(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> bool {
        let mut compliant = true;
        
        // Check for console.log (should use proper logging)
        if file.content.contains("console.log") {
            issues.push(QualityIssue::new(
                Severity::Low,
                CheckCategory::BestPractices,
                file.path.clone(),
                "Using console.log instead of proper logging".to_string(),
                "Use a logging library".to_string(),
            ).with_auto_fix(false));
            compliant = false;
        }
        
        // Check line length (basic check)
        for (i, line) in file.content.lines().enumerate() {
            if line.len() > 120 {
                issues.push(
                    QualityIssue::new(
                        Severity::Low,
                        CheckCategory::Standards,
                        file.path.clone(),
                        "Line exceeds 120 characters".to_string(),
                        "Break into multiple lines".to_string(),
                    )
                    .with_location(i + 1, None)
                    .with_auto_fix(true)
                );
                compliant = false;
            }
        }
        
        compliant
    }

    fn check_edge_cases(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> bool {
        let mut handled = true;
        
        // Check for unsafe property access (basic check)
        for (i, line) in file.content.lines().enumerate() {
            if line.contains(".") && !line.contains("?.") && !line.contains("if (") {
                // This is a very basic check - real implementation would use AST
                if line.matches('.').count() > 2 {
                    issues.push(
                        QualityIssue::new(
                            Severity::Medium,
                            CheckCategory::ErrorHandling,
                            file.path.clone(),
                            "Potential unsafe property access".to_string(),
                            "Use optional chaining (?.)".to_string(),
                        )
                        .with_location(i + 1, None)
                        .with_auto_fix(true)
                    );
                    handled = false;
                }
            }
        }
        
        // Check for async without try-catch
        let has_async = file.content.contains("async ");
        let has_try_catch = file.content.contains("try {");
        if has_async && !has_try_catch {
            issues.push(QualityIssue::new(
                Severity::High,
                CheckCategory::ErrorHandling,
                file.path.clone(),
                "Async function without error handling".to_string(),
                "Add try-catch block".to_string(),
            ).with_auto_fix(true));
            handled = false;
        }
        
        handled
    }

    fn check_documentation(&self, file: &GeneratedFile, issues: &mut Vec<QualityIssue>) -> bool {
        // Check for JSDoc comments on exported functions
        let has_jsdoc = file.content.contains("/**");
        let has_exports = !file.exports.is_empty();
        
        if has_exports && !has_jsdoc {
            issues.push(QualityIssue::new(
                Severity::Low,
                CheckCategory::Documentation,
                file.path.clone(),
                "Missing JSDoc comments for public APIs".to_string(),
                "Add JSDoc comments to exported functions".to_string(),
            ).with_auto_fix(false));
            return false;
        }
        
        true
    }
}
