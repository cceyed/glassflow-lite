# Architect Agent Documentation

## Overview
The Architect Agent is the first agent in the pipeline. It transforms vague user ideas into detailed, unambiguous technical specifications that other agents can execute.

## Implementation Status
✅ **Complete** - Core functionality implemented and validated

## Module Structure

```
src-tauri/src/agents/architect/
├── mod.rs           # Main Architect implementation
├── analysis.rs      # Specification parsing and analysis
├── questions.rs     # Question generation for clarification
├── design.rs        # Architecture design and planning
└── confidence.rs    # Confidence score calculation
```

## Key Components

### 1. Architect (mod.rs)
Main agent orchestrator with two primary methods:
- **`analyze_and_design()`** - Main entry point for specification analysis
- **`process_answers()`** - Process user answers to clarification questions

### 2. Analysis Module (analysis.rs)
Parses user specifications and extracts:
- Project intent (WebApp, MobileApp, API, etc.)
- Explicit requirements
- Implicit requirements
- Ambiguities that need clarification
- Missing critical information

### 3. Questions Module (questions.rs)
Generates clarifying questions:
- Prioritizes high-impact ambiguities
- Combines related questions
- Offers sensible defaults
- Limits to max 7 questions

### 4. Design Module (design.rs)
Creates architecture plans including:
- Technology stack selection
- File structure planning
- Component hierarchy
- Architecture decisions with reasoning
- Dependency management

### 5. Confidence Module (confidence.rs)
Calculates confidence scores based on:
- **Spec Clarity** (25%) - How clear the specification is
- **Technical Feasibility** (25%) - How feasible the tech stack is
- **Architecture Soundness** (20%) - How sound the architecture is
- **Completeness** (20%) - How complete the plan is
- **Risk Assessment** (10%) - How risky the approach is

## State Machine

```
IDLE → ANALYZING → QUESTIONING → DESIGNING → COMPLETE
         ↓              ↓            ↓
       ERROR ←────────ERROR ←──────ERROR
```

### States (from blueprint)
1. **IDLE** - Waiting for user specification
2. **ANALYZING** - Parsing and analyzing input
3. **QUESTIONING** - Asking clarifying questions (if needed)
4. **DESIGNING** - Creating architecture plan
5. **COMPLETE** - Architecture finalized
6. **ERROR** - Cannot proceed

## Configuration

```rust
pub struct ArchitectConfig {
    pub max_questions: usize,           // Default: 7
    pub confidence_threshold: f32,      // Default: 85.0
    pub enable_reasoning_stream: bool,  // Default: true
}
```

## Usage Example

```rust
use crate::agents::architect::{Architect, ArchitectConfig};
use crate::llm::client::create_llm_client();

// Create architect with default config
let llm_client = create_llm_client();
let mut architect = Architect::new(llm_client);

// Analyze specification
match architect.analyze_and_design(spec).await {
    Ok(plan) => {
        println!("Architecture complete!");
        println!("Confidence: {:.1}%", plan.confidence.overall);
    }
    Err(e) if e.to_string().contains("Clarification needed") => {
        // Questions will be asked via IPC
        println!("Need to ask clarification questions");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## IPC Integration

The Architect Agent is integrated with the frontend via IPC commands in `src-tauri/src/ipc/commands.rs`:

### Commands
1. **architect_analyze** - Start specification analysis
2. **architect_get_state** - Get current agent state
3. **architect_answer** - Submit answer to clarification question
4. **architect_cancel** - Cancel current operation
5. **architect_export_plan** - Export architecture plan
6. **architect_retry** - Retry with modified specification

## Validation

Run the validation script:
```bash
bash scripts/validate-architect.sh
```

This validates:
- ✅ Rust compilation
- ✅ Rust tests
- ✅ TypeScript compilation
- ✅ All required backend files present

## Blueprint Compliance

The implementation follows the Phase 2 blueprint (`docs/archive/phase2.md`):
- ✅ State machine with all required states
- ✅ Specification analysis with LLM
- ✅ Ambiguity detection
- ✅ Question generation (max 7, prioritized)
- ✅ Architecture design
- ✅ Confidence calculation with breakdown
- ✅ IPC commands for frontend integration
- ✅ Reasoning stream support

## Key Features

### Specification Analysis
- Extracts explicit and implicit requirements
- Detects project intent automatically
- Identifies ambiguities and missing information
- Categorizes requirements by type

### Question Strategy
- Prioritizes high-impact questions first
- Combines related questions
- Offers sensible defaults
- Limits to 7 questions maximum
- Multiple question types (single choice, yes/no, free text)

### Architecture Design
- Selects appropriate tech stack
- Plans file structure
- Designs component hierarchy
- Documents architecture decisions
- Calculates dependencies

### Confidence Scoring
- Multi-factor confidence calculation
- Weighted scoring (clarity, feasibility, soundness, completeness, risk)
- Detailed breakdown for transparency
- Threshold-based quality gates

## Next Steps

1. **Frontend Integration**: Connect UI components to IPC commands
2. **Testing**: Add integration tests for full workflow
3. **Question UI**: Implement question display components
4. **Plan Visualization**: Create architecture plan viewer
