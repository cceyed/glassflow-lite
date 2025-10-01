# Data Model: Architect Agent System

**Feature**: 002-phase-2-architect  
**Date**: 2025-09-30  
**Status**: Complete

## Core Entities

### 1. AgentState (Enum)

**Purpose**: Represents the current state of the Architect Agent

**States**:
```rust
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
```

**State Transitions**:
- IDLE → ANALYZING (user submits spec)
- ANALYZING → QUESTIONING (ambiguities found)
- ANALYZING → DESIGNING (spec complete)
- ANALYZING → ERROR (invalid spec)
- QUESTIONING → QUESTIONING (more questions)
- QUESTIONING → DESIGNING (all answered)
- QUESTIONING → ERROR (contradictions)
- DESIGNING → COMPLETE (success)
- DESIGNING → ERROR (infeasible)
- COMPLETE → IDLE (auto after 2s)
- ERROR → IDLE (cancel)
- ERROR → ANALYZING (retry)

**Validation Rules**:
- Cannot transition IDLE → COMPLETE directly
- Cannot skip ANALYZING phase
- Must validate spec before ANALYZING → DESIGNING

---

### 2. SpecificationAnalysis

**Purpose**: Parsed and analyzed user specification

**Fields**:
```rust
pub struct SpecificationAnalysis {
    pub raw_input: String,
    pub intent: ProjectIntent,
    pub explicit_requirements: Vec<Requirement>,
    pub implicit_requirements: Vec<Requirement>,
    pub ambiguities: Vec<Ambiguity>,
    pub missing_critical_info: Vec<String>,
    pub technical_keywords: Vec<String>,
    pub confidence: f32,
}

pub enum ProjectIntent {
    WebApp,
    MobileApp,
    DesktopApp,
    API,
    Library,
    CLI,
    Unknown,
}
```

**Validation Rules**:
- `raw_input` must not be empty
- `confidence` must be in range [0.0, 100.0]
- `ambiguities` sorted by impact (High → Medium → Low)
- `explicit_requirements` must have at least one item for valid spec

**Relationships**:
- Contains multiple `Requirement` objects
- Contains multiple `Ambiguity` objects
- Used by `Designing` state

---

### 3. Requirement

**Purpose**: Individual functional or non-functional requirement

**Fields**:
```rust
pub struct Requirement {
    pub category: RequirementCategory,
    pub content: String,
    pub priority: Priority,
    pub source: Source,
}

pub enum RequirementCategory {
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

pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

pub enum Source {
    Explicit,  // User specified
    Inferred,  // Agent inferred
}
```

**Validation Rules**:
- `content` must not be empty
- `category` must be valid enum value
- Critical requirements must be resolved before DESIGNING

---

### 4. Ambiguity

**Purpose**: Identified unclear aspect requiring clarification

**Fields**:
```rust
pub struct Ambiguity {
    pub category: RequirementCategory,
    pub description: String,
    pub impact: Impact,
    pub suggested_questions: Vec<String>,
}

pub enum Impact {
    High,    // Affects core architecture
    Medium,  // Affects implementation details
    Low,     // Nice-to-have clarification
}
```

**Validation Rules**:
- `description` must not be empty
- `suggested_questions` must have 1-3 items
- High-impact ambiguities must be resolved before DESIGNING (unless safely defaulted)

**Relationships**:
- Generates `Question` objects
- Sorted by impact for prioritization

---

### 5. Question

**Purpose**: Clarifying question presented to user

**Fields**:
```rust
pub struct Question {
    pub id: String,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<QuestionOption>>,
    pub recommended_answer: Option<String>,
    pub reasoning: Option<String>,
    pub impact: Impact,
}

pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    YesNo,
    FreeText,
    ConfirmationWithDefault,
}

pub struct QuestionOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}
```

**Validation Rules**:
- `id` must be unique
- `text` must not be empty
- SingleChoice/MultipleChoice must have `options`
- YesNo must not have `options`
- Max 7 questions per session

**Relationships**:
- Generated from `Ambiguity` objects
- Answers stored in `Questioning` state

---

### 6. ArchitecturePlan

**Purpose**: Complete architecture specification output

**Fields**:
```rust
pub struct ArchitecturePlan {
    pub project_name: String,
    pub project_type: ProjectIntent,
    pub tech_stack: TechStack,
    pub architecture_pattern: ArchitecturePattern,
    pub file_structure: FileStructure,
    pub components: Vec<Component>,
    pub state_management: StateManagement,
    pub routing: Option<Routing>,
    pub styling: StylingApproach,
    pub testing: TestingStrategy,
    pub build_config: BuildConfig,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
    pub decisions: Vec<ArchitectureDecision>,
    pub confidence: ConfidenceBreakdown,
}

pub struct TechStack {
    pub framework: String,
    pub language: String,
    pub runtime: Option<String>,
    pub bundler: Option<String>,
}

pub enum ArchitecturePattern {
    MVC,
    MVVM,
    Atomic,
    FeatureBased,
    DomainDriven,
    Layered,
}
```

**Validation Rules**:
- `project_name` must be valid identifier (alphanumeric + hyphens)
- `tech_stack.framework` and `tech_stack.language` required
- `file_structure` must have at least one directory
- `components` must have at least one component
- `decisions` must have at least one decision
- `confidence.overall` must be in [0.0, 100.0]

**Relationships**:
- Contains multiple `Component` objects
- Contains multiple `ArchitectureDecision` objects
- Contains `ConfidenceBreakdown`
- Stored in `Complete` state

---

### 7. Component

**Purpose**: Individual UI or logic component in the architecture

**Fields**:
```rust
pub struct Component {
    pub name: String,
    pub purpose: String,
    pub file_path: String,
    pub props: Vec<Prop>,
    pub state: Vec<StateItem>,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

pub struct Prop {
    pub name: String,
    pub type_: String,
    pub required: bool,
}

pub struct StateItem {
    pub name: String,
    pub type_: String,
    pub initial_value: Option<String>,
}
```

**Validation Rules**:
- `name` must be unique within plan
- `file_path` must be valid relative path
- `purpose` must not be empty
- Parent-child relationships must be consistent (no cycles)

---

### 8. ArchitectureDecision

**Purpose**: Documented design decision with rationale

**Fields**:
```rust
pub struct ArchitectureDecision {
    pub id: String,
    pub category: String,
    pub decision: String,
    pub reasoning: String,
    pub alternatives_considered: Vec<String>,
    pub impact: Impact,
    pub confidence: f32,
}
```

**Validation Rules**:
- `id` must be unique
- `decision` and `reasoning` must not be empty
- `alternatives_considered` should have at least one item
- `confidence` must be in [0.0, 1.0]

**Relationships**:
- Part of `ArchitecturePlan`
- Provides transparency for reviewers

---

### 9. ConfidenceBreakdown

**Purpose**: Detailed confidence metrics

**Fields**:
```rust
pub struct ConfidenceBreakdown {
    pub overall: f32,
    pub spec_clarity: f32,
    pub technical_feasibility: f32,
    pub architecture_soundness: f32,
    pub completeness: f32,
    pub risk_assessment: f32,
}
```

**Calculation**:
```rust
impl ConfidenceBreakdown {
    pub fn calculate(analysis: &SpecificationAnalysis, plan: &ArchitecturePlan) -> Self {
        let spec_clarity = 1.0 - (0.1 * analysis.ambiguities.len() as f32 
            + 0.15 * analysis.high_impact_ambiguities().len() as f32);
        let spec_clarity = spec_clarity.clamp(0.0, 1.0);
        
        let feasibility = calculate_feasibility(plan);  // 0-1
        let soundness = calculate_soundness(plan);      // 0-1
        let completeness = calculate_completeness(plan); // 0-1
        let risk = calculate_risk(plan);                // 0-1
        
        let overall = (spec_clarity * 0.25 
            + feasibility * 0.25 
            + soundness * 0.20 
            + completeness * 0.20 
            + risk * 0.10) * 100.0;
        
        Self {
            overall: overall.clamp(0.0, 100.0),
            spec_clarity: spec_clarity * 100.0,
            technical_feasibility: feasibility * 100.0,
            architecture_soundness: soundness * 100.0,
            completeness: completeness * 100.0,
            risk_assessment: risk * 100.0,
        }
    }
}
```

**Validation Rules**:
- All fields must be in [0.0, 100.0]
- Overall must match weighted formula

---

### 10. ReasoningEntry

**Purpose**: Real-time reasoning log entry

**Fields**:
```rust
pub struct ReasoningEntry {
    pub timestamp: DateTime<Utc>,
    pub phase: ArchitectPhase,
    pub type_: ReasoningType,
    pub content: String,
    pub confidence: Option<f32>,
    pub related_to: Option<String>,
}

pub enum ArchitectPhase {
    Analyzing,
    Questioning,
    Designing,
}

pub enum ReasoningType {
    Observation,
    Analysis,
    Decision,
    Question,
    Conclusion,
}
```

**Validation Rules**:
- `timestamp` must be valid UTC time
- `content` must not be empty
- `confidence` if present must be in [0.0, 100.0]

**Relationships**:
- Streamed to UI in real-time
- Buffered (last 100 entries)

---

## Data Flow

```
User Input (String)
    ↓
SpecificationAnalysis (parse + analyze)
    ↓
Ambiguity[] (if unclear)
    ↓
Question[] (generate clarifications)
    ↓
Answers (HashMap<String, String>)
    ↓
ArchitecturePlan (design)
    ↓
JSON Output (export)
```

## Persistence

**State Persistence**:
- File: `~/.glassflow/architect-state.json`
- Format: JSON serialization of `ArchitectState`
- Frequency: On every state transition
- Purpose: Resume after interruption

**Plan Export**:
- Format: JSON
- Schema: `ArchitecturePlan` structure
- Destination: User-specified path or passed to Engineer Agent

---

## Validation Summary

**Pre-ANALYZING**:
- Spec must not be empty

**Pre-DESIGNING**:
- All high-impact ambiguities resolved or defaulted
- `missing_critical_info` is empty
- At least one explicit requirement

**Pre-COMPLETE**:
- Plan has all required sections
- Confidence calculated
- At least one architecture decision documented

**Error Conditions**:
- Empty spec → ERROR
- Contradictory requirements → ERROR
- LLM parse failures (after retries) → ERROR
- Technically infeasible architecture → ERROR
