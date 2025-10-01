// T026 & T027: CodeQualityCheck and QualityIssue structs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityCheck {
    pub syntax_valid: bool,
    pub types_correct: bool,
    pub imports_resolved: bool,
    pub exports_valid: bool,
    pub style_compliant: bool,
    pub edge_cases_handled: bool,
    pub documented: bool,
    pub issues: Vec<QualityIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub id: String,
    pub severity: Severity,
    pub category: CheckCategory,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub description: String,
    pub impact: String,
    pub suggestion: String,
    pub auto_fixable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_context: Option<CodeContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum CheckCategory {
    Syntax,
    TypeSafety,
    Imports,
    Exports,
    CodeSmells,
    Security,
    Performance,
    BestPractices,
    Standards,
    Documentation,
    ErrorHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeContext {
    pub before: String,
    pub problematic_line: String,
    pub after: String,
}

impl CodeQualityCheck {
    pub fn new() -> Self {
        Self {
            syntax_valid: false,
            types_correct: false,
            imports_resolved: false,
            exports_valid: false,
            style_compliant: false,
            edge_cases_handled: false,
            documented: false,
            issues: Vec::new(),
        }
    }

    pub fn all_checks_passed(&self) -> bool {
        self.syntax_valid
            && self.types_correct
            && self.imports_resolved
            && self.exports_valid
            && self.style_compliant
            && self.edge_cases_handled
            && self.documented
            && self.issues.is_empty()
    }

    pub fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Critical)
    }

    pub fn has_high_issues(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::High)
    }

    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.issues.iter().filter(|i| i.severity == severity).count()
    }

    pub fn auto_fixable_issues(&self) -> Vec<&QualityIssue> {
        self.issues.iter().filter(|i| i.auto_fixable).collect()
    }
}

impl Default for CodeQualityCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityIssue {
    pub fn new(
        severity: Severity,
        category: CheckCategory,
        file: String,
        description: String,
        suggestion: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            severity,
            category,
            file,
            line: None,
            column: None,
            description,
            impact: String::new(),
            suggestion,
            auto_fixable: false,
            code_context: None,
        }
    }

    pub fn with_location(mut self, line: usize, column: Option<usize>) -> Self {
        self.line = Some(line);
        self.column = column;
        self
    }

    pub fn with_impact(mut self, impact: String) -> Self {
        self.impact = impact;
        self
    }

    pub fn with_auto_fix(mut self, auto_fixable: bool) -> Self {
        self.auto_fixable = auto_fixable;
        self
    }

    pub fn with_context(mut self, context: CodeContext) -> Self {
        self.code_context = Some(context);
        self
    }
}

impl Severity {
    pub fn penalty_score(&self) -> f32 {
        match self {
            Severity::Critical => 20.0,
            Severity::High => 10.0,
            Severity::Medium => 5.0,
            Severity::Low => 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    pub issue_id: String,
    pub file: String,
    pub description: String,
    pub before: String,
    pub after: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConfidenceBreakdown {
    pub overall: f32,
    pub code_quality: f32,
    pub type_safety: f32,
    pub security: f32,
    pub performance: f32,
    pub maintainability: f32,
}
