PHASE 3: ENGINEER AGENT
3.1 Engineer Agent Overview
Purpose
The Engineer agent takes the architecture plan from Architect and implements it by generating clean, functional code. It's the "builder" that turns designs into reality.
Core Responsibilities

Implement code based on architecture plan
Generate all necessary files
Follow coding standards and best practices
Handle edge cases and error states
Document code with comments
Ensure type safety and correctness

Success Criteria

Code compiles without errors
Follows architecture plan exactly
Adheres to coding standards
Handles edge cases properly
Well-documented and readable
High confidence score (>85%)


3.2 Engineer State Machine
States
IDLE

Waiting for architecture plan
LED: ○ (hollow circle, dim)
Display: "Ready"

ANALYZING_PLAN

Reading architecture plan from Architect
Understanding requirements and structure
Planning implementation approach
LED: ◐ (half circle, pulsing)
Display: "Analyzing architecture..."
Duration: 2-5 seconds

GENERATING_CODE

Writing code for each file
Creating components, functions, types
Setting up configuration files
LED: ● (filled, bright white + glow)
Display: "Generating code..."
Duration: 10-30 seconds
Progress: File-by-file progress

REVIEWING

Self-review of generated code
Checking for consistency
Validating against plan
LED: ◑ (inverted half, pulsing)
Display: "Reviewing implementation..."
Duration: 3-8 seconds

COMPLETE

All files generated successfully
Code ready for Quality review
LED: ✓ (checkmark, white with green glow)
Display: "Code generation complete"
Shows: File count, line count, confidence

ERROR

Cannot implement part of plan
Technical constraint violated
Dependency conflict
LED: ✗ (X mark, white with red glow)
Display: Error message
Options: Retry, modify plan, skip

State Transitions
IDLE → ANALYZING_PLAN
  Trigger: Architect completes plan
  Validation: Plan is valid and complete
  
ANALYZING_PLAN → GENERATING_CODE
  Trigger: Plan understood
  Validation: Implementation path clear
  
ANALYZING_PLAN → ERROR
  Trigger: Plan is unimplementable
  Validation: Technical impossibility detected
  
GENERATING_CODE → REVIEWING
  Trigger: All files generated
  Validation: No generation errors
  
GENERATING_CODE → ERROR
  Trigger: Cannot generate file
  Validation: Code generation failed
  
REVIEWING → COMPLETE
  Trigger: Self-review passed
  Validation: Code meets standards
  
REVIEWING → GENERATING_CODE
  Trigger: Issues found
  Validation: Needs fixes
  
COMPLETE → IDLE
  Trigger: Auto-transition after handoff
  
ERROR → IDLE
  Trigger: User cancels
  
ERROR → ANALYZING_PLAN
  Trigger: Retry with modified plan

3.3 Engineer Intelligence
Plan Analysis
ruststruct PlanAnalysis {
    architecture_plan: ArchitecturePlan,
    implementation_strategy: ImplementationStrategy,
    file_generation_order: Vec<String>,
    dependencies_graph: DependencyGraph,
    complexity_estimate: ComplexityEstimate,
    potential_issues: Vec<PotentialIssue>,
}

struct ImplementationStrategy {
    approach: CodeGenApproach,
    file_order: FileGenerationOrder,
    templating_strategy: TemplatingStrategy,
    reuse_patterns: Vec<PatternTemplate>,
}

enum CodeGenApproach {
    TopDown,      // Start with root, work down
    BottomUp,     // Start with leaves, work up
    InsideOut,    // Start with core, expand outward
}

enum FileGenerationOrder {
    DependencyFirst,  // Generate dependencies first
    Sequential,       // Follow file structure order
    Parallel,         // Generate independent files together
}

struct ComplexityEstimate {
    total_files: usize,
    total_estimated_lines: usize,
    complex_files: Vec<String>,
    estimated_duration: Duration,
}

struct PotentialIssue {
    category: IssueCategory,
    description: String,
    severity: Severity,
    mitigation: String,
}

enum IssueCategory {
    CircularDependency,
    TypeIncompatibility,
    MissingDependency,
    PerformanceConcern,
    SecurityConcern,
}
Code Generation
File Generation Process:
ruststruct CodeGenerator {
    llm_client: Arc<LLMClient>,
    template_library: TemplateLibrary,
    style_guide: StyleGuide,
    current_context: GenerationContext,
}

struct GenerationContext {
    project: ArchitecturePlan,
    generated_files: HashMap<String, GeneratedFile>,
    current_file: Option<String>,
    imports_registry: ImportsRegistry,
    type_registry: TypeRegistry,
}

struct GeneratedFile {
    path: String,
    content: String,
    language: Language,
    lines: usize,
    imports: Vec<Import>,
    exports: Vec<Export>,
    types: Vec<TypeDefinition>,
    confidence: f32,
}

impl CodeGenerator {
    async fn generate_file(&mut self, file_template: &FileTemplate) -> Result<GeneratedFile> {
        self.emit_reasoning(&format!("Generating {}", file_template.path));
        
        // Build context for this file
        let context = self.build_file_context(file_template)?;
        
        // Generate code using LLM
        let code = self.generate_code_with_llm(file_template, &context).await?;
        
        // Parse and validate
        let parsed = self.parse_code(&code, file_template.language)?;
        
        // Check quality
        let quality = self.check_code_quality(&parsed)?;
        
        // Calculate confidence
        let confidence = self.calculate_file_confidence(&quality);
        
        let generated = GeneratedFile {
            path: file_template.path.clone(),
            content: code,
            language: file_template.language,
            lines: parsed.lines,
            imports: parsed.imports,
            exports: parsed.exports,
            types: parsed.types,
            confidence,
        };
        
        // Register in context
        self.register_file(&generated)?;
        
        self.emit_reasoning(&format!("✓ Generated {} ({} lines, {}% confidence)", 
            file_template.path, parsed.lines, confidence));
        
        Ok(generated)
    }
    
    async fn generate_code_with_llm(
        &self, 
        file_template: &FileTemplate,
        context: &FileContext,
    ) -> Result<String> {
        let prompt = self.build_generation_prompt(file_template, context);
        
        let mut code = String::new();
        
        // Stream response
        let mut stream = self.llm_client.stream_message(prompt).await?;
        
        while let Some(chunk) = stream.next().await {
            code.push_str(&chunk?);
            self.emit_progress_update(&code);
        }
        
        Ok(code)
    }
    
    fn build_generation_prompt(
        &self,
        file_template: &FileTemplate,
        context: &FileContext,
    ) -> String {
        format!(
            "Generate {language} code for: {path}\n\n\
             Purpose: {purpose}\n\n\
             Architecture:\n{architecture}\n\n\
             Dependencies:\n{dependencies}\n\n\
             Style Guide:\n{style}\n\n\
             Requirements:\n\
             - Follow TypeScript best practices\n\
             - Include proper type definitions\n\
             - Add JSDoc comments\n\
             - Handle edge cases\n\
             - Export necessary types/functions\n\n\
             Generate ONLY the code, no explanations:",
            language = file_template.language,
            path = file_template.path,
            purpose = file_template.purpose,
            architecture = context.architecture_summary,
            dependencies = context.dependencies,
            style = self.style_guide.summary(),
        )
    }
}
Code Quality Checks
ruststruct CodeQualityCheck {
    syntax_valid: bool,
    types_correct: bool,
    imports_resolved: bool,
    exports_valid: bool,
    style_compliant: bool,
    edge_cases_handled: bool,
    documented: bool,
    issues: Vec<QualityIssue>,
}

struct QualityIssue {
    severity: Severity,
    category: QualityCategory,
    line: Option<usize>,
    description: String,
    suggestion: String,
}

enum QualityCategory {
    Syntax,
    Type,
    Import,
    Style,
    Logic,
    Performance,
    Security,
}

impl CodeGenerator {
    fn check_code_quality(&self, parsed: &ParsedCode) -> Result<CodeQualityCheck> {
        let mut issues = Vec::new();
        
        // Check syntax
        let syntax_valid = self.check_syntax(parsed, &mut issues);
        
        // Check types
        let types_correct = self.check_types(parsed, &mut issues);
        
        // Check imports
        let imports_resolved = self.check_imports(parsed, &mut issues);
        
        // Check exports
        let exports_valid = self.check_exports(parsed, &mut issues);
        
        // Check style
        let style_compliant = self.check_style(parsed, &mut issues);
        
        // Check edge cases
        let edge_cases_handled = self.check_edge_cases(parsed, &mut issues);
        
        // Check documentation
        let documented = self.check_documentation(parsed, &mut issues);
        
        Ok(CodeQualityCheck {
            syntax_valid,
            types_correct,
            imports_resolved,
            exports_valid,
            style_compliant,
            edge_cases_handled,
            documented,
            issues,
        })
    }
    
    fn check_types(&self, parsed: &ParsedCode, issues: &mut Vec<QualityIssue>) -> bool {
        let mut all_valid = true;
        
        // Check for 'any' types
        for usage in &parsed.any_type_usages {
            issues.push(QualityIssue {
                severity: Severity::Medium,
                category: QualityCategory::Type,
                line: Some(usage.line),
                description: "Using 'any' type loses type safety".to_string(),
                suggestion: "Use specific type or generic".to_string(),
            });
            all_valid = false;
        }
        
        // Check for missing return types
        for func in &parsed.functions {
            if func.return_type.is_none() {
                issues.push(QualityIssue {
                    severity: Severity::Low,
                    category: QualityCategory::Type,
                    line: Some(func.line),
                    description: format!("Function '{}' missing return type", func.name),
                    suggestion: "Add explicit return type".to_string(),
                });
            }
        }
        
        all_valid
    }
    
    fn check_edge_cases(&self, parsed: &ParsedCode, issues: &mut Vec<QualityIssue>) -> bool {
        let mut handled = true;
        
        // Check for null/undefined handling
        for access in &parsed.property_accesses {
            if !access.is_safe {
                issues.push(QualityIssue {
                    severity: Severity::High,
                    category: QualityCategory::Logic,
                    line: Some(access.line),
                    description: "Potential null/undefined access".to_string(),
                    suggestion: "Use optional chaining (?.) or null check".to_string(),
                });
                handled = false;
            }
        }
        
        // Check for error handling
        for async_call in &parsed.async_calls {
            if !async_call.has_error_handling {
                issues.push(QualityIssue {
                    severity: Severity::High,
                    category: QualityCategory::Logic,
                    line: Some(async_call.line),
                    description: "Async call without error handling".to_string(),
                    suggestion: "Wrap in try-catch or use .catch()".to_string(),
                });
                handled = false;
            }
        }
        
        handled
    }
}
Confidence Calculation
rustfn calculate_engineer_confidence(
    quality: &CodeQualityCheck,
    plan_adherence: f32,
) -> f32 {
    // Base quality score
    let quality_score = calculate_quality_score(quality);
    
    // Plan adherence (did we follow architecture?)
    let adherence_score = plan_adherence;
    
    // Issue penalty
    let issue_penalty = calculate_issue_penalty(&quality.issues);
    
    // Weighted average
    let base_confidence = 
        quality_score * 0.50 +
        adherence_score * 0.30 +
        (1.0 - issue_penalty) * 0.20;
    
    (base_confidence * 100.0).clamp(0.0, 100.0)
}

fn calculate_quality_score(quality: &CodeQualityCheck) -> f32 {
    let checks = [
        quality.syntax_valid,
        quality.types_correct,
        quality.imports_resolved,
        quality.exports_valid,
        quality.style_compliant,
        quality.edge_cases_handled,
        quality.documented,
    ];
    
    let passed = checks.iter().filter(|&&c| c).count() as f32;
    passed / checks.len() as f32
}

fn calculate_issue_penalty(issues: &[QualityIssue]) -> f32 {
    let mut penalty = 0.0;
    
    for issue in issues {
        penalty += match issue.severity {
            Severity::Critical => 0.20,
            Severity::High => 0.10,
            Severity::Medium => 0.05,
            Severity::Low => 0.02,
        };
    }
    
    penalty.min(0.8) // Cap at 80% penalty
}

3.4 Engineer UI Components
Engineer Card (Active)
┌────────────────────────────────────────┐
│ ● ENGINEER                        95%  │
│ Writing src/components/TodoList.tsx    │
│ ▓▓▓▓▓▓▓▓▓▓▓░░░░ 73% (12/17 files)    │
└────────────────────────────────────────┘
Engineer Panel (Expanded)
┌─────────────────────────────────────────────────────┐
│ [●] ENGINEER                       Confidence: 95%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Phase: Generating Code                              │
│ Current File: src/components/TodoList.tsx           │
│ Progress: 12/17 files (73%)                         │
│ Elapsed: 14.3s | Estimated: 5.2s remaining          │
│                                                      │
│ Reasoning:                                          │
│                                                      │
│ ○ Analyzed architecture plan                        │
│   • 17 files to generate                            │
│   • Using feature-based structure                   │
│   • TypeScript with strict mode                     │
│   ↳ Plan Clarity: 98%                              │
│                                                      │
│ ○ Generated core files                              │
│   ✓ src/App.tsx (67 lines, 98%)                    │
│   ✓ src/main.tsx (23 lines, 99%)                   │
│   ✓ src/store/todoStore.ts (80 lines, 96%)         │
│   ✓ src/types/todo.ts (15 lines, 100%)             │
│   ↳ Core: Complete                                  │
│                                                      │
│ ○ Generating components                             │
│   ✓ TodoItem.tsx (54 lines, 97%)                   │
│   ✓ AddTodo.tsx (43 lines, 95%)                    │
│   ● TodoList.tsx (in progress...)                   │
│     • Line 45 of ~89                                │
│     • Adding event handlers                         │
│     • Implementing filter logic                     │
│   ↳ Components: 67% complete                        │
│                                                      │
│ ○ Pending files                                     │
│   ○ src/components/TodoFilter.tsx                   │
│   ○ src/components/TodoStats.tsx                    │
│   ○ src/styles/index.css                            │
│   ○ package.json, tsconfig.json, vite.config.ts    │
│   ○ README.md, .gitignore                           │
│   ↳ Remaining: 5 files                              │
│                                                      │
│ Quality Checks (so far):                            │
│ • Syntax: ✓ All valid                               │
│ • Types: ✓ No 'any' types                          │
│ • Imports: ✓ All resolved                           │
│ • Style: ⚠ 2 minor issues (auto-fixable)           │
│ • Edge cases: ✓ Handled                             │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [Pause] [View Generated Code] [Skip File]          │
└─────────────────────────────────────────────────────┘
Real-time Code Preview
┌─────────────────────────────────────────────────────┐
│ Generating: src/components/TodoList.tsx             │
├─────────────────────────────────────────────────────┤
│                                                      │
│  1  import React from 'react';                      │
│  2  import { useTodoStore } from '../store/todoS... │
│  3  import TodoItem from './TodoItem';              │
│  4  import type { Todo } from '../types/todo';      │
│  5                                                   │
│  6  export default function TodoList() {            │
│  7    const { todos, filter } = useTodoStore();     │
│  8                                                   │
│  9    const filteredTodos = React.useMemo(() => {   │
│ 10      if (filter === 'all') return todos;         │
│ 11      if (filter === 'active') return todos.fi... │
│ 12      return todos.filter(t => t.completed);      │
│ 13    }, [todos, filter]);                          │
│ 14                                                   │
│ 15    return (                                       │
│ 16      <div className="todo-list">                 │
│ 17        {filteredTodos.map(todo => (              │
│ 18          <TodoItem key={todo.id} todo={todo} />  │
│ 19        ))}                                        │
│ 20      </div>█                                      │
│                                                      │
│ [Writing line 21...]                                │
│                                                      │
└─────────────────────────────────────────────────────┘

3.5 Engineer Backend Implementation
rust// src-tauri/src/agents/engineer.rs

pub struct Engineer {
    llm_client: Arc<LLMClient>,
    code_generator: CodeGenerator,
    state: EngineerState,
    config: EngineerConfig,
}

pub struct EngineerConfig {
    pub streaming_enabled: bool,
    pub self_review_enabled: bool,
    pub max_retries: usize,
    pub style_guide: StyleGuide,
}

pub enum EngineerState {
    Idle,
    AnalyzingPlan {
        plan: ArchitecturePlan,
    },
    GeneratingCode {
        plan: ArchitecturePlan,
        files_completed: Vec<GeneratedFile>,
        current_file: Option<String>,
        progress: f32,
    },
    Reviewing {
        generated_files: Vec<GeneratedFile>,
    },
    Complete {
        output: CodeOutput,
        duration: Duration,
    },
    Error {
        message: String,
        failed_file: Option<String>,
        recoverable: bool,
    },
}

pub struct CodeOutput {
    pub files: Vec<GeneratedFile>,
    pub total_lines: usize,
    pub confidence: ConfidenceBreakdown,
    pub quality_report: QualityReport,
}

impl Engineer {
    pub async fn implement_plan(&mut self, plan: ArchitecturePlan) -> Result<CodeOutput> {
        let start_time = Instant::now();
        
        // Analyze plan
        self.state = EngineerState::AnalyzingPlan {
            plan: plan.clone(),
        };
        
        self.emit_reasoning("Analyzing architecture plan");
        let analysis = self.analyze_plan(&plan).await?;
        
        self.emit_reasoning(&format!(
            "Will generate {} files (~{} lines)",
            analysis.total_files,
            analysis.total_estimated_lines
        ));
        
        // Generate files
        self.state = EngineerState::GeneratingCode {
            plan: plan.clone(),
            files_completed: Vec::new(),
            current_file: None,
            progress: 0.0,
        };
        
        let mut generated_files = Vec::new();
        let total_files = plan.file_structure.files.len();
        
        for (index, file_template) in plan.file_structure.files.iter().enumerate() {
            self.emit_reasoning(&format!("Generating {} ({}/{})", 
                file_template.path, index + 1, total_files));
            
            if let EngineerState::GeneratingCode { current_file, .. } = &mut self.state {
                *current_file = Some(file_template.path.clone());
            }
            
            match self.code_generator.generate_file(file_template).await {
                Ok(file) => {
                    generated_files.push(file);
                    self.emit_file_complete(&file_template.path);
                },
                Err(e) => {
                    self.state = EngineerState::Error {
                        message: e.to_string(),
                        failed_file: Some(file_template.path.clone()),
                        recoverable: true,
                    };
                    return Err(e);
                }
            }
            
            let progress = (index + 1) as f32 / total_files as f32;
            self.emit_progress(progress);
        }
        
        // Self-review
        if self.config.self_review_enabled {
            self.state = EngineerState::Reviewing {
                generated_files: generated_files.clone(),
            };
            
            self.emit_reasoning("Reviewing generated code");
            self.self_review(&mut generated_files).await?;
        }
        
        // Calculate metrics
        let total_lines = generated_files.iter()
            .map(|f| f.lines)
            .sum();
        
        let confidence = self.calculate_overall_confidence(&generated_files);
        let quality_report = self.generate_quality_report(&generated_files);
        
        let output = CodeOutput {
            files: generated_files,
            total_lines,
            confidence,
            quality_report,
        };
        
        let duration = start_time.elapsed();
        
        self.state = EngineerState::Complete {
            output: output.clone(),
            duration,
        };
        
        self.emit_reasoning(&format!(
            "Code generation complete ({} files, {} lines, {}s, {}% confidence)",
            output.files.len(),
            output.total_lines,
            duration.as_secs(),
            confidence.overall
        ));
        
        Ok(output)
    }
    
    async fn self_review(&self, files: &mut Vec<GeneratedFile>) -> Result<()> {
        let mut issues_found = 0;
        let mut fixes_applied = 0;
        
        for file in files.iter_mut() {
            let quality = self.code_generator.check_code_quality_parsed(&file)?;
            
            if !quality.issues.is_empty() {
                issues_found += quality.issues.len();
                self.emit_reasoning(&format!(
                    "Found {} issues in {}",
                    quality.issues.len(),
                    file.path
                ));
                
                // Auto-fix minor issues
                for issue in &quality.issues {
                    if matches!(issue.severity, Severity::Low | Severity::Medium) {
                        if let Ok(fixed) = self.auto_fix_issue(file, issue).await {
                            file.content = fixed;
                            fixes_applied += 1;
                        }
                    }
                }
            }
        }
        
        if fixes_applied > 0 {
            self.emit_reasoning(&format!(
                "Auto-fixed {}/{} issues",
                fixes_applied,
                issues_found
            ));
        }
        
        Ok(())
    }
    
    fn emit_reasoning(&self, content: &str) {
        // Emit via Tauri events
    }
    
    fn emit_progress(&self, progress: f32) {
        // Update progress bar
    }
    
    fn emit_file_complete(&self, path: &str) {
        // Notify UI of file completion
    }
}

3.6 Testing Engineer Agent
rust#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_simple_file_generation() {
        let engineer = create_test_engineer();
        let file_template = FileTemplate {
            path: "src/App.tsx".to_string(),
            purpose: "Root component".to_string(),
            estimated_lines: 50,
            language: Language::TypeScript,
        };
        
        let result = engineer.code_generator
            .generate_file(&file_template)
            .await
            .unwrap();
        
        assert!(result.content.contains("import React"));
        assert!(result.content.contains("export default"));
        assert!(result.lines > 0);
    }
    
    #[tokio::test]
    async fn test_type_safety() {
        let engineer = create_test_engineer();
        let file_template = create_component_template();
        
        let result = engineer.code_generator
            .generate_file(&file_template)
            .await
            .unwrap();
        
        // Should not use 'any' type
        assert!(!result.content.contains(": any"));
        
        // Should have proper types
        assert!(result.types.len() > 0);
    }
    
    #[tokio::test]
    async fn test_error_handlingparse_simple_spec() {
        let architect = create_test_architect();
        let spec = "Build a todo app with React";
        
        let analysis = architect.parse_specification(spec).await.unwrap();
        
        assert_eq!(analysis.intent, ProjectIntent::WebApp);
        assert!(analysis.explicit_requirements.iter()
            .any(|r| r.content.contains("React")));
    }
    
    #[tokio::test]
    async fn test_