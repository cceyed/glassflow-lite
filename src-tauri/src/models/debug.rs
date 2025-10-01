// Debug Agent Models
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugReport {
    pub build_passed: bool,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub bugs: Vec<Bug>,
    pub fixes_applied: Vec<DebugAppliedFix>,
    pub confidence: DebugConfidenceBreakdown,
    #[serde(skip)]
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BugSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BugCategory {
    BuildError,
    RuntimeError,
    StateMutation,
    MemoryLeak,
    TypeMismatch,
    NullPointer,
    UnhandledError,
    PerformanceIssue,
    AccessibilityIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bug {
    pub id: String,
    pub severity: BugSeverity,
    pub category: BugCategory,
    pub file: String,
    pub line: usize,
    pub description: String,
    pub error_message: String,
    pub code_context: String,
    pub impact: String,
    pub auto_fixable: bool,
    pub fixed: bool,
    pub suggested_fixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugAppliedFix {
    pub bug_id: String,
    pub description: String,
    pub before: String,
    pub after: String,
    pub confidence: f32,
    pub validation_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfidenceBreakdown {
    pub overall: f32,
    pub build: f32,
    pub runtime: f32,
    pub error_handling: f32,
    pub state_management: f32,
    pub edge_cases: f32,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub bug: Option<Bug>,
}
