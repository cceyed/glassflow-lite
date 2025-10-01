// T038-T039: AutoFixer - auto-fix minor quality issues
use crate::models::{GeneratedFile, QualityIssue, Severity, AppliedFix};
use anyhow::Result;

pub struct AutoFixer;

impl AutoFixer {
    pub fn apply_fixes(file: &mut GeneratedFile, issues: &[QualityIssue]) -> Result<Vec<AppliedFix>> {
        let mut applied_fixes = Vec::new();
        
        for issue in issues {
            if !issue.auto_fixable {
                continue;
            }
            
            // Only auto-fix Low and Medium severity issues
            if !matches!(issue.severity, Severity::Low | Severity::Medium) {
                continue;
            }
            
            if let Some(fix) = Self::try_fix(file, issue)? {
                applied_fixes.push(fix);
            }
        }
        
        Ok(applied_fixes)
    }

    fn try_fix(file: &mut GeneratedFile, issue: &QualityIssue) -> Result<Option<AppliedFix>> {
        let before = file.content.clone();
        
        let fixed = match issue.description.as_str() {
            desc if desc.contains("Line exceeds") => Self::fix_long_lines(&file.content),
            desc if desc.contains("optional chaining") => Self::fix_optional_chaining(&file.content),
            desc if desc.contains("error handling") => Self::fix_error_handling(&file.content),
            desc if desc.contains("semicolon") => Self::fix_semicolons(&file.content),
            _ => None,
        };
        
        if let Some(after) = fixed {
            file.content = after.clone();
            file.lines = file.content.lines().count();
            
            Ok(Some(AppliedFix {
                issue_id: issue.id.clone(),
                file: file.path.clone(),
                description: issue.description.clone(),
                before,
                after,
                confidence: 0.9,
            }))
        } else {
            Ok(None)
        }
    }

    fn fix_long_lines(content: &str) -> Option<String> {
        let mut fixed_lines = Vec::new();
        let mut changed = false;
        
        for line in content.lines() {
            if line.len() > 120 {
                // Simple fix: break at commas or operators
                if let Some(comma_pos) = line[..100].rfind(',') {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    fixed_lines.push(line[..=comma_pos].to_string());
                    fixed_lines.push(format!("{}{}", indent, line[comma_pos + 1..].trim()));
                    changed = true;
                    continue;
                }
            }
            fixed_lines.push(line.to_string());
        }
        
        if changed {
            Some(fixed_lines.join("\n"))
        } else {
            None
        }
    }

    fn fix_optional_chaining(content: &str) -> Option<String> {
        // Replace unsafe property access with optional chaining
        // This is a simplified implementation
        let mut fixed = content.to_string();
        let mut changed = false;
        
        // Look for patterns like: obj.prop.method()
        // Replace with: obj?.prop?.method()
        // This is very basic - real implementation would use AST
        
        if fixed.contains(".") && !fixed.contains("?.") {
            // Simple heuristic replacement
            fixed = fixed.replace(".then(", "?.then(");
            fixed = fixed.replace(".catch(", "?.catch(");
            changed = true;
        }
        
        if changed {
            Some(fixed)
        } else {
            None
        }
    }

    fn fix_error_handling(content: &str) -> Option<String> {
        // Wrap async functions in try-catch
        let mut fixed = String::new();
        let mut in_async_function = false;
        let mut brace_count = 0;
        let mut changed = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Detect async function start
            if trimmed.contains("async ") && trimmed.contains("{") {
                in_async_function = true;
                brace_count = 1;
                fixed.push_str(line);
                fixed.push('\n');
                
                // Add try block
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                fixed.push_str(&format!("{}  try {{\n", indent));
                changed = true;
                continue;
            }
            
            if in_async_function {
                brace_count += trimmed.matches('{').count() as i32;
                brace_count -= trimmed.matches('}').count() as i32;
                
                if brace_count == 0 {
                    // End of function - add catch block
                    let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                    fixed.push_str(&format!("{}  }} catch (error) {{\n", indent));
                    fixed.push_str(&format!("{}    console.error('Error:', error);\n", indent));
                    fixed.push_str(&format!("{}    throw error;\n", indent));
                    fixed.push_str(&format!("{}  }}\n", indent));
                    in_async_function = false;
                }
            }
            
            fixed.push_str(line);
            fixed.push('\n');
        }
        
        if changed {
            Some(fixed)
        } else {
            None
        }
    }

    fn fix_semicolons(content: &str) -> Option<String> {
        // Add missing semicolons
        let mut fixed_lines = Vec::new();
        let mut changed = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Check if line should end with semicolon
            if !trimmed.is_empty()
                && !trimmed.ends_with(';')
                && !trimmed.ends_with('{')
                && !trimmed.ends_with('}')
                && !trimmed.ends_with(',')
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("/*")
                && !trimmed.starts_with('*')
            {
                fixed_lines.push(format!("{};", line));
                changed = true;
            } else {
                fixed_lines.push(line.to_string());
            }
        }
        
        if changed {
            Some(fixed_lines.join("\n"))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_long_lines() {
        let long_line = "const result = someFunction(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15);";
        let fixed = AutoFixer::fix_long_lines(long_line);
        assert!(fixed.is_some());
        let fixed = fixed.unwrap();
        assert!(fixed.lines().all(|l| l.len() <= 120));
    }

    #[test]
    fn test_fix_semicolons() {
        let code = "const x = 5\nconst y = 10";
        let fixed = AutoFixer::fix_semicolons(code);
        assert!(fixed.is_some());
        assert!(fixed.unwrap().contains("const x = 5;"));
    }
}
