// Debug Confidence Calculator
use crate::models::{Bug, BugSeverity, DebugConfidenceBreakdown, TestResult};
use crate::agents::debug::test_runner::BuildTestResult;

pub struct DebugConfidenceCalculator;

impl DebugConfidenceCalculator {
    pub fn calculate(
        build_result: &BuildTestResult,
        component_results: &[TestResult],
        state_results: &[TestResult],
        async_results: &[TestResult],
        bugs: &[Bug],
    ) -> DebugConfidenceBreakdown {
        let build_score = if build_result.passed { 1.0 } else { 0.0 };
        
        let runtime_score = Self::calculate_test_score(component_results);
        let error_handling_score = Self::calculate_test_score(async_results);
        let state_score = Self::calculate_test_score(state_results);
        let edge_case_score = 0.80; // Placeholder
        
        // Bug penalty
        let bug_penalty = Self::calculate_bug_penalty(bugs);
        
        // Weighted average
        let overall = (
            build_score * 0.30 +
            runtime_score * 0.30 +
            error_handling_score * 0.20 +
            state_score * 0.10 +
            edge_case_score * 0.10
        ) * (1.0 - bug_penalty);
        
        DebugConfidenceBreakdown {
            overall: (overall * 100.0).clamp(0.0, 100.0),
            build: build_score * 100.0,
            runtime: runtime_score * 100.0,
            error_handling: error_handling_score * 100.0,
            state_management: state_score * 100.0,
            edge_cases: edge_case_score * 100.0,
        }
    }
    
    fn calculate_test_score(results: &[TestResult]) -> f32 {
        if results.is_empty() {
            return 1.0;
        }
        
        let passed = results.iter().filter(|r| r.passed).count();
        passed as f32 / results.len() as f32
    }
    
    fn calculate_bug_penalty(bugs: &[Bug]) -> f32 {
        let mut penalty: f32 = 0.0;
        
        for bug in bugs {
            if !bug.fixed {
                penalty += match bug.severity {
                    BugSeverity::Critical => 0.40,
                    BugSeverity::High => 0.20,
                    BugSeverity::Medium => 0.10,
                    BugSeverity::Low => 0.03,
                };
            }
        }
        
        penalty.min(0.90)
    }
}
