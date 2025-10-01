// Confidence Calculator - calculates quality confidence scores
use crate::models::{QualityIssue, Severity, GeneratedFile, QualityConfidenceBreakdown};

pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    pub fn calculate(issues: &[QualityIssue], files: &[GeneratedFile]) -> QualityConfidenceBreakdown {
        let code_quality = Self::calculate_code_quality(issues, files);
        let type_safety = Self::calculate_type_safety(issues);
        let security = Self::calculate_security(issues);
        let performance = Self::calculate_performance(issues);
        let maintainability = Self::calculate_maintainability(issues, files);

        // Issue penalty
        let issue_penalty = Self::calculate_issue_penalty(issues);

        // Weighted average
        let overall = (
            code_quality * 0.25 +
            type_safety * 0.25 +
            security * 0.25 +
            performance * 0.15 +
            maintainability * 0.10
        ) * (1.0 - issue_penalty);

        QualityConfidenceBreakdown {
            overall: (overall * 100.0).clamp(0.0, 100.0),
            code_quality: (code_quality * 100.0),
            type_safety: (type_safety * 100.0),
            security: (security * 100.0),
            performance: (performance * 100.0),
            maintainability: (maintainability * 100.0),
        }
    }

    fn calculate_code_quality(issues: &[QualityIssue], _files: &[GeneratedFile]) -> f32 {
        let code_smell_issues = issues.iter()
            .filter(|i| matches!(i.category, crate::models::CheckCategory::CodeSmells))
            .count();

        // Base score starts at 1.0, reduce by issues
        let penalty = (code_smell_issues as f32 * 0.05).min(0.5);
        (1.0 - penalty).max(0.5)
    }

    fn calculate_type_safety(issues: &[QualityIssue]) -> f32 {
        let type_issues = issues.iter()
            .filter(|i| matches!(i.category, crate::models::CheckCategory::TypeSafety))
            .count();

        if type_issues == 0 {
            1.0
        } else {
            (1.0 - (type_issues as f32 * 0.1)).max(0.3)
        }
    }

    fn calculate_security(issues: &[QualityIssue]) -> f32 {
        let security_issues = issues.iter()
            .filter(|i| matches!(i.category, crate::models::CheckCategory::Security))
            .count();

        if security_issues == 0 {
            1.0
        } else {
            // Security issues are heavily penalized
            (1.0 - (security_issues as f32 * 0.3)).max(0.0)
        }
    }

    fn calculate_performance(issues: &[QualityIssue]) -> f32 {
        let perf_issues = issues.iter()
            .filter(|i| matches!(i.category, crate::models::CheckCategory::Performance))
            .count();

        if perf_issues == 0 {
            1.0
        } else {
            (1.0 - (perf_issues as f32 * 0.08)).max(0.4)
        }
    }

    fn calculate_maintainability(issues: &[QualityIssue], files: &[GeneratedFile]) -> f32 {
        let avg_file_size = files.iter().map(|f| f.lines).sum::<usize>() as f32 / files.len() as f32;
        
        let maintainability_issues = issues.iter()
            .filter(|i| matches!(
                i.category,
                crate::models::CheckCategory::CodeSmells | 
                crate::models::CheckCategory::BestPractices
            ))
            .count();

        let size_penalty = if avg_file_size > 200.0 { 0.1 } else { 0.0 };
        let issue_penalty = maintainability_issues as f32 * 0.05;

        (1.0 - size_penalty - issue_penalty).max(0.5)
    }

    fn calculate_issue_penalty(issues: &[QualityIssue]) -> f32 {
        let mut penalty: f32 = 0.0;

        for issue in issues {
            penalty += match issue.severity {
                Severity::Critical => 0.30, // 30% penalty per critical
                Severity::High => 0.15,     // 15% penalty per high
                Severity::Medium => 0.05,   // 5% penalty per medium
                Severity::Low => 0.01,      // 1% penalty per low
            };
        }

        penalty.min(0.80) // Cap at 80% total penalty
    }
}
