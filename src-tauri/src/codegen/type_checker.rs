// T035: TypeChecker - validate TypeScript types using TypeScript Compiler API (via Command)
use crate::models::GeneratedFile;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use anyhow::Result;

pub struct TypeChecker;

impl TypeChecker {
    pub fn check_types(file: &GeneratedFile) -> Result<TypeCheckResult> {
        // Write content to temporary file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(file.content.as_bytes())?;
        let temp_path = temp_file.path();
        
        // Run TypeScript compiler
        let output = Command::new("npx")
            .args(&[
                "tsc",
                "--noEmit",
                "--strict",
                "--skipLibCheck",
                temp_path.to_str().unwrap(),
            ])
            .output()?;
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let errors = Self::parse_tsc_output(&stderr, &stdout);
        
        Ok(TypeCheckResult {
            passed: output.status.success(),
            errors,
        })
    }

    fn parse_tsc_output(stderr: &str, stdout: &str) -> Vec<TypeCheckError> {
        let mut errors = Vec::new();
        let combined = format!("{}\n{}", stderr, stdout);
        
        for line in combined.lines() {
            if line.contains("error TS") {
                errors.push(TypeCheckError {
                    message: line.to_string(),
                    line: Self::extract_line_number(line),
                });
            }
        }
        
        errors
    }

    fn extract_line_number(line: &str) -> Option<usize> {
        // Extract line number from TypeScript error format: "file.ts(10,5): error TS..."
        if let Some(start) = line.find('(') {
            if let Some(end) = line[start..].find(',') {
                if let Ok(num) = line[start + 1..start + end].parse() {
                    return Some(num);
                }
            }
        }
        None
    }

    pub fn has_any_types(content: &str) -> bool {
        // Check for 'any' type usage
        content.contains(": any") || content.contains("<any>") || content.contains("any[]")
    }

    pub fn has_explicit_return_types(content: &str) -> bool {
        // Simple heuristic: check if functions have return types
        // This is a basic implementation
        for line in content.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("function ") || trimmed.contains(" function ")) 
                && !trimmed.contains("): ") 
                && !trimmed.contains("=>") {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct TypeCheckResult {
    pub passed: bool,
    pub errors: Vec<TypeCheckError>,
}

#[derive(Debug)]
pub struct TypeCheckError {
    pub message: String,
    pub line: Option<usize>,
}
