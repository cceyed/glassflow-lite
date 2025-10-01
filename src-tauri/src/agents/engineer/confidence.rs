// T040-T043: Confidence calculation for Engineer Agent
use crate::models::{GeneratedFile, QualityIssue, Severity, EngineerConfidenceBreakdown, EngineerArchitecturePlan};

pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    /// T040: Calculate overall confidence
    pub fn calculate_confidence(
        files: &[GeneratedFile],
        issues: &[QualityIssue],
        plan: &EngineerArchitecturePlan,
    ) -> EngineerConfidenceBreakdown {
        let quality_score = Self::calculate_quality_score(files, issues);
        let plan_adherence = Self::calculate_plan_adherence(files, plan);
        let issue_penalty = Self::calculate_issue_penalty(issues);
        
        EngineerConfidenceBreakdown::new(quality_score, plan_adherence, issue_penalty)
    }

    /// T041: Calculate quality score (0-100)
    pub fn calculate_quality_score(files: &[GeneratedFile], issues: &[QualityIssue]) -> f32 {
        if files.is_empty() {
            return 0.0;
        }
        
        // Base score from file confidence
        let avg_file_confidence: f32 = files.iter().map(|f| f.confidence).sum::<f32>() / files.len() as f32;
        let base_score = avg_file_confidence * 100.0;
        
        // Penalty for issues
        let critical_count = issues.iter().filter(|i| matches!(i.severity, Severity::Critical)).count();
        let high_count = issues.iter().filter(|i| matches!(i.severity, Severity::High)).count();
        
        let issue_penalty = (critical_count as f32 * 15.0 + high_count as f32 * 7.0).min(40.0);
        
        // Quality checks bonus
        let has_types = files.iter().any(|f| !f.types.is_empty());
        let has_exports = files.iter().any(|f| !f.exports.is_empty());
        let has_imports = files.iter().any(|f| !f.imports.is_empty());
        
        let bonus = if has_types { 5.0 } else { 0.0 }
            + if has_exports { 5.0 } else { 0.0 }
            + if has_imports { 5.0 } else { 0.0 };
        
        (base_score - issue_penalty + bonus).clamp(0.0, 100.0)
    }

    /// T042: Calculate plan adherence (0-100)
    pub fn calculate_plan_adherence(files: &[GeneratedFile], plan: &EngineerArchitecturePlan) -> f32 {
        let expected_files = plan.file_structure.files.len();
        let generated_files = files.len();
        
        if expected_files == 0 {
            return 100.0;
        }
        
        // File count match
        let file_count_score = if generated_files == expected_files {
            40.0
        } else {
            (generated_files as f32 / expected_files as f32 * 40.0).min(40.0)
        };
        
        // Line count accuracy
        let expected_lines: usize = plan.file_structure.files.iter().map(|f| f.estimated_lines).sum();
        let actual_lines: usize = files.iter().map(|f| f.lines).sum();
        
        let line_accuracy = if expected_lines > 0 {
            let ratio = actual_lines as f32 / expected_lines as f32;
            // Ideal is 0.8-1.2x estimated
            if ratio >= 0.8 && ratio <= 1.2 {
                30.0
            } else if ratio >= 0.6 && ratio <= 1.5 {
                20.0
            } else {
                10.0
            }
        } else {
            0.0
        };
        
        // Dependency adherence
        let mut dependency_score: f32 = 30.0;
        for file in files {
            for import in &file.imports {
                // Check if import matches plan dependencies
                let is_valid = plan.dependencies.iter().any(|d| import.source.contains(&d.name))
                    || import.source.starts_with("./")
                    || import.source.starts_with("../");
                
                if !is_valid {
                    dependency_score -= 2.0;
                }
            }
        }
        dependency_score = dependency_score.max(0.0);
        
        (file_count_score + line_accuracy + dependency_score).clamp(0.0, 100.0)
    }

    /// T043: Calculate issue penalty (0-100, capped at 80)
    pub fn calculate_issue_penalty(issues: &[QualityIssue]) -> f32 {
        let critical_count = issues.iter().filter(|i| matches!(i.severity, Severity::Critical)).count();
        let high_count = issues.iter().filter(|i| matches!(i.severity, Severity::High)).count();
        let medium_count = issues.iter().filter(|i| matches!(i.severity, Severity::Medium)).count();
        let low_count = issues.iter().filter(|i| matches!(i.severity, Severity::Low)).count();
        
        let penalty = critical_count as f32 * 20.0
            + high_count as f32 * 10.0
            + medium_count as f32 * 5.0
            + low_count as f32 * 2.0;
        
        penalty.min(80.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Language, ProjectIntent, TechStack, FileStructure, ArchitecturePattern};

    fn create_test_file() -> GeneratedFile {
        GeneratedFile {
            path: "test.ts".to_string(),
            content: "export const test = 42;".to_string(),
            language: Language::TypeScript,
            lines: 1,
            imports: vec![],
            exports: vec![],
            types: vec![],
            confidence: 0.9,
        }
    }

    fn create_test_plan() -> EngineerArchitecturePlan {
        EngineerArchitecturePlan {
            project_name: "test".to_string(),
            project_intent: ProjectIntent::WebApp,
            tech_stack: TechStack {
                language: "TypeScript".to_string(),
                runtime: "Node.js".to_string(),
                framework: None,
                styling: None,
                state_management: None,
                testing: None,
            },
            architecture_pattern: ArchitecturePattern::Atomic,
            file_structure: FileStructure {
                directories: vec![],
                files: vec![],
            },
            component_hierarchy: vec![],
            dependencies: vec![],
            architecture_decisions: vec![],
            confidence_breakdown: None,
        }
    }

    #[test]
    fn test_quality_score_perfect() {
        let files = vec![create_test_file()];
        let issues = vec![];
        
        let score = ConfidenceCalculator::calculate_quality_score(&files, &issues);
        assert!(score > 80.0);
    }

    #[test]
    fn test_issue_penalty() {
        let issues = vec![
            QualityIssue::new(
                Severity::Critical,
                crate::models::CheckCategory::Syntax,
                "test.ts".to_string(),
                "Test".to_string(),
                "Fix".to_string(),
            ),
        ];
        
        let penalty = ConfidenceCalculator::calculate_issue_penalty(&issues);
        assert_eq!(penalty, 20.0);
    }

    #[test]
    fn test_overall_confidence() {
        let files = vec![create_test_file()];
        let issues = vec![];
        let plan = create_test_plan();
        
        let confidence = ConfidenceCalculator::calculate_confidence(&files, &issues, &plan);
        assert!(confidence.overall > 0.0);
        assert!(confidence.overall <= 100.0);
    }
}
