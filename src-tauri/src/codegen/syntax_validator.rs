// T036: SyntaxValidator - validate syntax using TypeScript Compiler API (via Command)
use crate::models::GeneratedFile;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use anyhow::Result;

pub struct SyntaxValidator;

impl SyntaxValidator {
    pub fn validate_syntax(file: &GeneratedFile) -> Result<SyntaxValidationResult> {
        // Write content to temporary file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(file.content.as_bytes())?;
        let temp_path = temp_file.path();
        
        // Run TypeScript compiler for syntax check only
        let output = Command::new("npx")
            .args(&[
                "tsc",
                "--noEmit",
                "--allowJs",
                "--checkJs=false",
                temp_path.to_str().unwrap(),
            ])
            .output()?;
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let errors = Self::parse_syntax_errors(&stderr, &stdout);
        
        Ok(SyntaxValidationResult {
            valid: output.status.success() && errors.is_empty(),
            errors,
        })
    }

    fn parse_syntax_errors(stderr: &str, stdout: &str) -> Vec<SyntaxError> {
        let mut errors = Vec::new();
        let combined = format!("{}\n{}", stderr, stdout);
        
        for line in combined.lines() {
            // Look for syntax errors
            if line.contains("error") && (
                line.contains("Unexpected") ||
                line.contains("Expected") ||
                line.contains("Cannot find") ||
                line.contains("';' expected")
            ) {
                errors.push(SyntaxError {
                    message: line.to_string(),
                    line: Self::extract_line_number(line),
                    column: Self::extract_column_number(line),
                });
            }
        }
        
        errors
    }

    fn extract_line_number(line: &str) -> Option<usize> {
        if let Some(start) = line.find('(') {
            if let Some(comma) = line[start..].find(',') {
                if let Ok(num) = line[start + 1..start + comma].parse() {
                    return Some(num);
                }
            }
        }
        None
    }

    fn extract_column_number(line: &str) -> Option<usize> {
        if let Some(start) = line.find('(') {
            if let Some(comma) = line[start..].find(',') {
                if let Some(end) = line[start + comma..].find(')') {
                    let col_str = &line[start + comma + 1..start + comma + end];
                    if let Ok(num) = col_str.parse() {
                        return Some(num);
                    }
                }
            }
        }
        None
    }

    pub fn check_basic_syntax(content: &str) -> Vec<String> {
        let mut issues = Vec::new();
        
        // Check for balanced braces
        let open_braces = content.matches('{').count();
        let close_braces = content.matches('}').count();
        if open_braces != close_braces {
            issues.push(format!("Unbalanced braces: {} open, {} close", open_braces, close_braces));
        }
        
        // Check for balanced parentheses
        let open_parens = content.matches('(').count();
        let close_parens = content.matches(')').count();
        if open_parens != close_parens {
            issues.push(format!("Unbalanced parentheses: {} open, {} close", open_parens, close_parens));
        }
        
        // Check for balanced brackets
        let open_brackets = content.matches('[').count();
        let close_brackets = content.matches(']').count();
        if open_brackets != close_brackets {
            issues.push(format!("Unbalanced brackets: {} open, {} close", open_brackets, close_brackets));
        }
        
        issues
    }
}

#[derive(Debug)]
pub struct SyntaxValidationResult {
    pub valid: bool,
    pub errors: Vec<SyntaxError>,
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_syntax_check() {
        let valid_code = "function test() { return 42; }";
        let issues = SyntaxValidator::check_basic_syntax(valid_code);
        assert!(issues.is_empty());
        
        let invalid_code = "function test() { return 42;";
        let issues = SyntaxValidator::check_basic_syntax(invalid_code);
        assert!(!issues.is_empty());
    }
}
