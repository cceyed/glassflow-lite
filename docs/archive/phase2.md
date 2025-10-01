PHASE 2: ARCHITECT AGENT
2.1 Architect Agent Overview
Purpose
The Architect is the first agent in the pipeline. Its job is to transform vague user ideas into detailed, unambiguous technical specifications that other agents can execute.
Core Responsibilities

Parse user input and extract intent
Identify ambiguities and missing information
Question user to clarify requirements
Design system architecture and structure
Document comprehensive specification
Validate specification completeness

Success Criteria

Specifications are clear and unambiguous
All technical decisions are documented
Engineer can implement without clarification
Architecture is sound and follows best practices
Confidence score accurately reflects spec quality


2.2 Architect State Machine
States
IDLE

Waiting for user specification
LED: ○ (hollow circle, dim gray)
Display: "Ready"
Actions: None

ANALYZING

Reading and parsing user input
Identifying requirements and ambiguities
Planning questions to ask
LED: ◐ (half circle, pulsing white)
Display: "Analyzing specification..."
Duration: 1-5 seconds
Progress: Indeterminate spinner

QUESTIONING

Asking clarifying questions to user
Waiting for user responses
Building complete picture of requirements
LED: ◑ (inverted half, pulsing slower)
Display: "Waiting for clarification..."
Duration: Variable (user-dependent)
Progress: Question count (e.g., "3/7 questions answered")

DESIGNING

Creating architecture plan
Selecting technologies and patterns
Planning file structure
Documenting decisions
LED: ● (filled circle, bright white + glow)
Display: "Designing architecture..."
Duration: 5-15 seconds
Progress: 0-100% based on subtasks

COMPLETE

Architecture plan finalized
Specification document ready
LED: ✓ (checkmark, white with subtle green glow)
Display: "Architecture complete"
Shows: Confidence score, elapsed time

ERROR

Cannot proceed with current information
Specification is impossible/invalid
Critical issue found
LED: ✗ (X mark, white with subtle red glow)
Display: Error message
Actions: Retry, modify spec, cancel

State Transitions
IDLE → ANALYZING
  Trigger: User submits specification
  Validation: Specification is not empty
  
ANALYZING → QUESTIONING
  Trigger: Ambiguities identified
  Validation: At least one question to ask
  
ANALYZING → DESIGNING
  Trigger: Specification is clear enough
  Validation: All critical information present
  
ANALYZING → ERROR
  Trigger: Specification is invalid/impossible
  Validation: No viable path forward
  
QUESTIONING → QUESTIONING
  Trigger: More questions needed
  Validation: Previous answer received
  
QUESTIONING → DESIGNING
  Trigger: All questions answered
  Validation: Specification now complete
  
QUESTIONING → ERROR
  Trigger: User provides contradictory info
  Validation: Cannot resolve conflict
  
DESIGNING → COMPLETE
  Trigger: Architecture plan finished
  Validation: Plan meets quality threshold
  
DESIGNING → ERROR
  Trigger: Architecture is not feasible
  Validation: Technical constraints violated
  
COMPLETE → IDLE
  Trigger: Auto-transition after 2 seconds
  Or: User acknowledges completion
  
ERROR → IDLE
  Trigger: User cancels or restarts
  
ERROR → ANALYZING
  Trigger: User provides modified spec

2.3 Architect Intelligence
Input Analysis
Parse User Specification:
ruststruct SpecificationAnalysis {
    raw_input: String,
    intent: ProjectIntent,
    explicit_requirements: Vec<Requirement>,
    implicit_requirements: Vec<Requirement>,
    ambiguities: Vec<Ambiguity>,
    missing_critical_info: Vec<String>,
    technical_keywords: Vec<String>,
    confidence: f32,
}

enum ProjectIntent {
    WebApp,
    MobileApp,
    DesktopApp,
    API,
    Library,
    CLI,
    Unknown,
}

struct Requirement {
    category: RequirementCategory,
    content: String,
    priority: Priority,
    source: Source, // Explicit or Inferred
}

enum RequirementCategory {
    Framework,
    Language,
    Styling,
    StateManagement,
    Authentication,
    Database,
    Deployment,
    Testing,
    Other(String),
}

struct Ambiguity {
    category: RequirementCategory,
    description: String,
    impact: Impact, // High, Medium, Low
    suggested_questions: Vec<String>,
}
Example Analysis:
Input: "Build a todo app with React"
rustSpecificationAnalysis {
    raw_input: "Build a todo app with React",
    intent: ProjectIntent::WebApp,
    explicit_requirements: vec![
        Requirement {
            category: Framework,
            content: "React",
            priority: Critical,
            source: Explicit,
        },
        Requirement {
            category: Other("ProjectType"),
            content: "Todo application",
            priority: Critical,
            source: Explicit,
        }
    ],
    implicit_requirements: vec![
        Requirement {
            category: Language,
            content: "JavaScript or TypeScript",
            priority: High,
            source: Inferred,
        },
        Requirement {
            category: StateManagement,
            content: "Some form of state management",
            priority: High,
            source: Inferred,
        }
    ],
    ambiguities: vec![
        Ambiguity {
            category: Language,
            description: "TypeScript not specified",
            impact: High,
            suggested_questions: vec![
                "Would you like to use TypeScript?",
            ],
        },
        Ambiguity {
            category: StateManagement,
            description: "State management approach unclear",
            impact: High,
            suggested_questions: vec![
                "Which state management would you prefer?",
                "Options: useState, Redux, Zustand, Jotai",
            ],
        },
        Ambiguity {
            category: Styling,
            description: "CSS approach not specified",
            impact: Medium,
            suggested_questions: vec![
                "How should styling be handled?",
                "Options: Plain CSS, Tailwind, CSS Modules, Styled Components",
            ],
        },
    ],
    missing_critical_info: vec![
        "Data persistence strategy",
        "Build tool preference",
    ],
    technical_keywords: vec!["React", "todo", "app"],
    confidence: 45.0, // Low due to many ambiguities
}
Question Generation
Question Strategy:
ruststruct QuestionStrategy {
    max_questions: usize,           // Default: 7
    prioritize_high_impact: bool,   // Default: true
    combine_related: bool,          // Default: true
    offer_defaults: bool,           // Default: true
    style: QuestionStyle,
}

enum QuestionStyle {
    Detailed,   // Long explanations
    Balanced,   // Some context
    Minimal,    // Just the question
}
Question Types:
Single Choice:
Which state management would you prefer?
a) useState/useReducer (React built-in)
b) Redux Toolkit (robust, popular)
c) Zustand (lightweight)
d) Jotai (atomic)

> _
Multiple Choice:
Which features do you need? (select all that apply)
a) User authentication
b) Data persistence
c) Dark mode
d) Real-time updates
e) None of these

(Enter letters separated by commas, e.g., a,c,d)
> _
Yes/No:
Should this support mobile devices?
(y/n)
> _
Free Text:
Any specific design requirements or constraints?
(Press Enter to skip)
> _
Confirmation with Default:
I recommend using TypeScript for better type safety.
Proceed with TypeScript?
(Y/n) [Y is default]
> _
Architecture Design
Architecture Planning:
ruststruct ArchitecturePlan {
    project_name: String,
    project_type: ProjectType,
    tech_stack: TechStack,
    architecture_pattern: ArchitecturePattern,
    file_structure: FileStructure,
    components: Vec<Component>,
    state_management: StateManagement,
    routing: Option<Routing>,
    styling: StylingApproach,
    testing: TestingStrategy,
    build_config: BuildConfig,
    dependencies: Vec<Dependency>,
    dev_dependencies: Vec<Dependency>,
    decisions: Vec<ArchitectureDecision>,
    confidence: ConfidenceBreakdown,
}

struct TechStack {
    framework: String,         // React, Vue, Svelte, etc.
    language: String,          // JavaScript, TypeScript
    runtime: Option<String>,   // Node.js, Deno, Bun
    bundler: Option<String>,   // Vite, Webpack, Rollup
}

enum ArchitecturePattern {
    MVC,
    MVVM,
    Atomic,           // Atomic Design
    FeatureBased,     // Feature-first organization
    DomainDriven,     // DDD
    Layered,
}

struct FileStructure {
    root: String,
    directories: Vec<Directory>,
    files: Vec<FileTemplate>,
}

struct Directory {
    path: String,
    purpose: String,
    files: Vec<FileTemplate>,
}

struct FileTemplate {
    path: String,
    purpose: String,
    estimated_lines: usize,
    dependencies: Vec<String>,
}

struct Component {
    name: String,
    purpose: String,
    file_path: String,
    props: Vec<Prop>,
    state: Vec<StateItem>,
    children: Vec<String>,
    parent: Option<String>,
}

struct ArchitectureDecision {
    id: String,
    category: String,
    decision: String,
    reasoning: String,
    alternatives_considered: Vec<String>,
    impact: Impact,
    confidence: f32,
}

struct ConfidenceBreakdown {
    overall: f32,
    spec_clarity: f32,
    technical_feasibility: f32,
    architecture_soundness: f32,
    completeness: f32,
    risk_assessment: f32,
}
Example Architecture Plan:
Input: React + TypeScript todo app with Zustand and Tailwind
rustArchitecturePlan {
    project_name: "todo-app",
    project_type: WebApp,
    tech_stack: TechStack {
        framework: "React 18",
        language: "TypeScript",
        runtime: Some("Node.js"),
        bundler: Some("Vite"),
    },
    architecture_pattern: FeatureBased,
    file_structure: FileStructure {
        root: "src/",
        directories: vec![
            Directory {
                path: "src/components",
                purpose: "React components",
                files: vec![
                    FileTemplate {
                        path: "src/components/TodoList.tsx",
                        purpose: "List container for todos",
                        estimated_lines: 45,
                    },
                    FileTemplate {
                        path: "src/components/TodoItem.tsx",
                        purpose: "Individual todo item",
                        estimated_lines: 60,
                    },
                    // ... more files
                ],
            },
            Directory {
                path: "src/store",
                purpose: "Zustand store",
                files: vec![
                    FileTemplate {
                        path: "src/store/todoStore.ts",
                        purpose: "Todo state management",
                        estimated_lines: 80,
                    },
                ],
            },
            // ... more directories
        ],
    },
    decisions: vec![
        ArchitectureDecision {
            category: "State Management",
            decision: "Use Zustand",
            reasoning: "Lightweight, minimal boilerplate, TypeScript-friendly",
            alternatives_considered: vec!["Redux", "Jotai", "useState"],
            impact: High,
            confidence: 0.92,
        },
        ArchitectureDecision {
            category: "Styling",
            decision: "Use Tailwind CSS",
            reasoning: "User requested, rapid development, consistent design",
            alternatives_considered: vec!["CSS Modules", "Styled Components"],
            impact: Medium,
            confidence: 0.98,
        },
        // ... more decisions
    ],
    confidence: ConfidenceBreakdown {
        overall: 0.94,
        spec_clarity: 0.95,
        technical_feasibility: 0.98,
        architecture_soundness: 0.92,
        completeness: 0.90,
        risk_assessment: 0.95,
    },
}
Confidence Calculation
Formula:
rustfn calculate_architect_confidence(analysis: &SpecificationAnalysis, plan: &ArchitecturePlan) -> f32 {
    // Spec Clarity (0-1)
    let spec_clarity = calculate_spec_clarity(analysis);
    
    // Technical Feasibility (0-1)
    let feasibility = calculate_feasibility(plan);
    
    // Architecture Soundness (0-1)
    let soundness = calculate_soundness(plan);
    
    // Completeness (0-1)
    let completeness = calculate_completeness(plan);
    
    // Risk Assessment (0-1, higher is less risky)
    let risk = calculate_risk(plan);
    
    // Weighted average
    let base_confidence = 
        spec_clarity * 0.25 +
        feasibility * 0.25 +
        soundness * 0.20 +
        completeness * 0.20 +
        risk * 0.10;
    
    // Convert to percentage
    (base_confidence * 100.0).clamp(0.0, 100.0)
}

fn calculate_spec_clarity(analysis: &SpecificationAnalysis) -> f32 {
    let total_ambiguities = analysis.ambiguities.len() as f32;
    let high_impact = analysis.ambiguities.iter()
        .filter(|a| matches!(a.impact, Impact::High))
        .count() as f32;
    
    // More ambiguities = lower clarity
    // High impact ambiguities penalized more
    let penalty = (total_ambiguities * 0.1) + (high_impact * 0.15);
    (1.0 - penalty).max(0.0)
}

fn calculate_feasibility(plan: &ArchitecturePlan) -> f32 {
    // Check if tech stack is well-supported
    let stack_maturity = check_stack_maturity(&plan.tech_stack);
    
    // Check if dependencies are compatible
    let dep_compatibility = check_dependency_compatibility(&plan.dependencies);
    
    // Check if patterns are proven
    let pattern_proven = check_pattern_maturity(&plan.architecture_pattern);
    
    (stack_maturity + dep_compatibility + pattern_proven) / 3.0
}

fn calculate_soundness(plan: &ArchitecturePlan) -> f32 {
    // Check separation of concerns
    let separation = check_separation_of_concerns(plan);
    
    // Check scalability
    let scalability = check_scalability(plan);
    
    // Check maintainability
    let maintainability = check_maintainability(plan);
    
    (separation + scalability + maintainability) / 3.0
}
Reasoning Generation
Real-time Reasoning Updates:
ruststruct ReasoningEntry {
    timestamp: DateTime<Utc>,
    phase: ArchitectPhase,
    type_: ReasoningType,
    content: String,
    confidence: Option<f32>,
    related_to: Option<String>,
}

enum ArchitectPhase {
    Analyzing,
    Questioning,
    Designing,
}

enum ReasoningType {
    Observation,    // "User specified React"
    Analysis,       // "React requires build tool"
    Decision,       // "Choosing Vite as build tool"
    Question,       // "Need to know about state management"
    Conclusion,     // "Specification is complete"
}
Example Reasoning Stream:
[ARCHITECT] Starting analysis...

[ARCHITECT] → Observation: User wants todo app
  ↳ Project type: Web application
  ↳ Core features: Task management

[ARCHITECT] → Observation: React specified
  ↳ Framework: React 18 (latest stable)
  ↳ Implies: Component-based architecture

[ARCHITECT] → Analysis: Language not specified
  ↳ Options: JavaScript or TypeScript
  ↳ TypeScript recommended for maintainability
  ↳ Impact: High (affects entire codebase)

[ARCHITECT] → Decision: Will ask about TypeScript
  ↳ Reasoning: Critical for project structure
  ↳ Confidence: 95%

[ARCHITECT] → Analysis: State management unclear
  ↳ Todo apps need state for tasks
  ↳ Options: useState, Context, Zustand, Redux
  ↳ Impact: High (affects architecture)

[ARCHITECT] → Decision: Will ask about state management
  ↳ Reasoning: Affects component structure
  ↳ Confidence: 90%

[ARCHITECT] → Conclusion: Need clarification
  ↳ Critical questions: 2
  ↳ Optional questions: 3
  ↳ Transitioning to questioning phase

2.4 Architect UI Components
Architect Card (Collapsed)
┌────────────────────────────────────────┐
│ ◐ ARCHITECT                       94%  │
│ Designing architecture...              │
│ ▓▓▓▓▓▓▓▓▓▓▓░░░░ 78%                   │
└────────────────────────────────────────┘
Architect Panel (Expanded)
┌─────────────────────────────────────────────────────┐
│ [●] ARCHITECT                      Confidence: 94%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Phase: Designing Architecture                       │
│ Task: Planning component structure                  │
│ Elapsed: 8.2s                                       │
│                                                      │
│ Reasoning:                                          │
│                                                      │
│ ○ Analyzed specification                            │
│   • Framework: React 18                             │
│   • Language: TypeScript                            │
│   • State: Zustand                                  │
│   • Styling: Tailwind CSS                           │
│   ↳ Spec Clarity: 95%                              │
│                                                      │
│ ○ Designed component hierarchy                      │
│   • App (root)                                      │
│     ├─ TodoList (container)                         │
│     ├─ TodoItem (presentational)                    │
│     └─ AddTodo (form)                              │
│   ↳ Soundness: 92%                                 │
│                                                      │
│ ○ Planned file structure                            │
│   • src/components/ (UI components)                 │
│   • src/store/ (Zustand store)                      │
│   • src/types/ (TypeScript types)                   │
│   ↳ Completeness: 90%                              │
│                                                      │
│ ○ Selected build configuration                      │
│   • Vite (fast, modern)                             │
│   • TypeScript (type safety)                        │
│   • ESLint (code quality)                           │
│   ↳ Feasibility: 98%                               │
│                                                      │
│ ○ Currently: Finalizing architecture document       │
│   • Estimated completion: 2s                        │
│   ↳ Current Confidence: 94%                        │
│                                                      │
│ Decisions Made: 12                                  │
│ Files Planned: 17                                   │
│ Dependencies: 8                                     │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [View Full Plan] [Export Spec] [Modify]            │
└─────────────────────────────────────────────────────┘
Question Display
┌─────────────────────────────────────────────────────┐
│ [ARCHITECT] Clarification needed                    │
├─────────────────────────────────────────────────────┤
│                                                      │
│ Question 1 of 3                                     │
│                                                      │
│ Which state management would you prefer?            │
│                                                      │
│ a) useState/useReducer                              │
│    └─ Built into React, simple, good for small apps│
│                                                      │
│ b) Redux Toolkit                                    │
│    └─ Robust, widely used, best for complex state  │
│                                                      │
│ c) Zustand                                          │
│    └─ Lightweight, minimal boilerplate, modern     │
│                                                      │
│ d) Jotai                                            │
│    └─ Atomic state, very flexible, TypeScript-first│
│                                                      │
│ Recommendation: Zustand (c)                         │
│ Reason: Best balance for todo app complexity        │
│                                                      │
│ Your choice (a/b/c/d or press Enter for default):  │
│ > _                                                 │
│                                                      │
└─────────────────────────────────────────────────────┘

2.5 Architect Backend Implementation
Rust Service Structure
rust// src-tauri/src/agents/architect.rs

pub struct Architect {
    llm_client: Arc<LLMClient>,
    config: ArchitectConfig,
    state: ArchitectState,
}

pub struct ArchitectConfig {
    pub max_questions: usize,
    pub question_style: QuestionStyle,
    pub confidence_threshold: f32,
    pub enable_reasoning_stream: bool,
}

pub enum ArchitectState {
    Idle,
    Analyzing {
        spec: String,
        start_time: Instant,
    },
    Questioning {
        questions: Vec<Question>,
        answers: HashMap<String, String>,
        current_question_index: usize,
    },
    Designing {
        analysis: SpecificationAnalysis,
        progress: f32,
    },
    Complete {
        plan: ArchitecturePlan,
        duration: Duration,
    },
    Error {
        message: String,
        recoverable: bool,
    },
}

impl Architect {
    pub fn new(llm_client: Arc<LLMClient>, config: ArchitectConfig) -> Self {
        Self {
            llm_client,
            config,
            state: ArchitectState::Idle,
        }
    }
    
    pub async fn analyze_specification(&mut self, spec: String) -> Result<()> {
        self.state = ArchitectState::Analyzing {
            spec: spec.clone(),
            start_time: Instant::now(),
        };
        
        // Emit reasoning updates
        self.emit_reasoning("Starting specification analysis");
        
        // Parse specification using LLM
        let analysis = self.parse_specification(&spec).await?;
        
        self.emit_reasoning(&format!("Identified {} requirements", 
            analysis.explicit_requirements.len()));
        
        // Check if clarification needed
        if !analysis.ambiguities.is_empty() {
            self.generate_questions(analysis).await?;
        } else {
            self.design_architecture(analysis).await?;
        }
        
        Ok(())
    }
    
    async fn parse_specification(&self, spec: &str) -> Result<SpecificationAnalysis> {
        let prompt = format!(
            "Analyze this project specification and extract requirements:\n\n{}\n\n\
             Return JSON with: intent, explicit_requirements, implicit_requirements, \
             ambiguities, missing_critical_info",
            spec
        );
        
        let response = self.llm_client.send_message(prompt).await?;
        let analysis: SpecificationAnalysis = serde_json::from_str(&response)?;
        
        Ok(analysis)
    }
    
    async fn generate_questions(&mut self, analysis: SpecificationAnalysis) -> Result<()> {
        self.emit_reasoning("Generating clarification questions");
        
        // Sort ambiguities by impact
        let mut sorted_ambiguities = analysis.ambiguities.clone();
        sorted_ambiguities.sort_by(|a, b| b.impact.cmp(&a.impact));
        
        // Generate questions for high-impact items
        let questions: Vec<Question> = sorted_ambiguities
            .iter()
            .take(self.config.max_questions)
            .map(|amb| self.create_question(amb))
            .collect();
        
        self.state = ArchitectState::Questioning {
            questions: questions.clone(),
            answers: HashMap::new(),
            current_question_index: 0,
        };
        
        // Emit first question
        self.emit_question(&questions[0]);
        
        Ok(())
    }
    
    pub async fn process_answer(&mut self, answer: String) -> Result<()> {
        if let ArchitectState::Questioning { 
            questions, 
            answers, 
            current_question_index 
        } = &mut self.state {
            // Store answer
            let question_id = questions[*current_question_index].id.clone();
            answers.insert(question_id, answer.clone());
            
            self.emit_reasoning(&format!("Received answer: {}", answer));
            
            // Move to next question
            *current_question_index += 1;
            
            if *current_question_index < questions.len() {
                // More questions to ask
                self.emit_question(&questions[*current_question_index]);
            } else {
                // All questions answered, proceed to design
                self.emit_reasoning("All questions answered, designing architecture");
                
                // Build complete analysis with answers
                let complete_analysis = self.build_complete_analysis(questions, answers).await?;
                self.design_architecture(complete_analysis).await?;
            }
        }
        
        Ok(())
    }
    
    async fn design_architecture(&mut self, analysis: SpecificationAnalysis) -> Result<()> {
        let start_time = Instant::now();
        
        self.state = ArchitectState::Designing {
            analysis: analysis.clone(),
            progress: 0.0,
        };
        
        self.emit_reasoning("Designing system architecture");
        self.emit_progress(0.1);
        
        // Design tech stack
        let tech_stack = self.design_tech_stack(&analysis).await?;
        self.emit_reasoning("Selected technology stack");
        self.emit_progress(0.3);
        
        // Design file structure
        let file_structure = self.design_file_structure(&analysis, &tech_stack).await?;
        self.emit_reasoning("Planned file structure");
        self.emit_progress(0.5);
        
        // Design components
        let components = self.design_components(&analysis).await?;
        self.emit_reasoning(&format!("Designed {} components", components.len()));
        self.emit_progress(0.7);
        
        // Make architecture decisions
        let decisions = self.make_decisions(&analysis, &tech_stack).await?;
        self.emit_reasoning(&format!("Made {} architecture decisions", decisions.len()));
        self.emit_progress(0.9);
        
        // Calculate confidence
        let confidence = self.calculate_confidence(&analysis);
        self.emit_progress(1.0);
        
        // Build final plan
        let plan = ArchitecturePlan {
            tech_stack,
            file_structure,
            components,
            decisions,
            confidence,
            // ... other fields
        };
        
        let duration = start_time.elapsed();
        
        self.state = ArchitectState::Complete {
            plan: plan.clone(),
            duration,
        };
        
        self.emit_reasoning(&format!("Architecture complete ({}s, {}% confidence)", 
            duration.as_secs(), confidence.overall));
        
        self.emit_complete(plan);
        
        Ok(())
    }
    
    fn emit_reasoning(&self, content: &str) {
        // Emit to frontend via Tauri event system
        // Frontend displays in reasoning panel
    }
    
    fn emit_progress(&self, progress: f32) {
        // Update progress bar in UI
    }
    
    fn emit_question(&self, question: &Question) {
        // Display question to user in UI
    }
    
    fn emit_complete(&self, plan: ArchitecturePlan) {
        // Notify frontend of completion
        // Pass plan to next agent (Engineer)
    }
}
IPC Commands
rust#[tauri::command]
async fn architect_analyze(spec: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut architect = state.architect.lock().await;
    architect.analyze_specification(spec).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn architect_answer(answer: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut architect = state.architect.lock().await;
    architect.process_answer(answer).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn architect_get_state(state: State<'_, AppState>) -> Result<ArchitectState, String> {
    let architect = state.architect.lock().await;
    Ok(architect.state.clone())
}

2.6 Testing Architect Agent
Unit Tests
rust#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ambiguity_detection() {
        let architect = create_test_architect();
        let spec = "Build a todo app"; // Very vague
        
        let analysis = architect.parse_specification(spec).await.unwrap();
        
        assert!(!analysis.ambiguities.is_empty());
        assert!(analysis.ambiguities.iter()
            .any(|a| matches!(a.category, RequirementCategory::Framework)));
    }
    
    #[tokio::test]
    async fn test_confidence_calculation() {
        let architect = create_test_architect();
        
        // Clear spec should have high confidence
        let clear_analysis = create_clear_analysis();
        let confidence = architect.calculate_confidence(&clear_analysis);
        assert!(confidence.overall > 85.0);
        
        // Vague spec should have low confidence
        let vague_analysis = create_vague_analysis();
        let confidence = architect.calculate_confidence(&vague_analysis);
        assert!(confidence.overall < 60.0);
    }
    
    #[tokio::test]
    async fn test_question_generation() {
        let architect = create_test_architect();
        let analysis = create_ambiguous_analysis();
        
        let questions = architect.generate_questions_from_analysis(&analysis);
        
        assert!(!questions.is_empty());
        assert!(questions.len() <= architect.config.max_questions);
        
        // High impact questions should come first
        assert_eq!(questions[0].impact, Impact::High);
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let mut architect = create_test_architect();
        
        // Start idle
        assert!(matches!(architect.state, ArchitectState::Idle));
        
        // Analyze spec
        architect.analyze_specification("Build a React app".to_string()).await.unwrap();
        
        // Should transition to Questioning or Designing
        assert!(matches!(
            architect.state,
            ArchitectState::Questioning { .. } | ArchitectState::Designing { .. }
        ));
    }
}

2.7 Architect Success Metrics
Performance Metrics

Analysis time: < 5 seconds
Question generation: < 2 seconds
Architecture design: < 15 seconds
Total average time: < 25 seconds

Quality Metrics

Spec clarity detection accuracy: > 90%
Architecture soundness: > 85% confidence average
Question relevance: > 95% (user feedback)
Completeness: > 90% of specs need no follow-up

User Experience Metrics

Number of questions: < 7 average
User understands questions: > 95%
Spec completion rate: > 90%
User satisfaction: > 4.5/5