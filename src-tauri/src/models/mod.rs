//! Data Models Module
//!
//! Core data structures for the Architect and Engineer Agent systems.
//!
//! This module contains all the data models used throughout the multi-agent system,
//! including state management, specification analysis, questions, architecture plans,
//! code generation, quality checking, and confidence scoring.
//!
//! # Key Components
//!
//! ## Architect Agent
//! - [`AgentState`]: The 6-state machine (IDLE, ANALYZING, QUESTIONING, DESIGNING, COMPLETE, ERROR)
//! - [`SpecificationAnalysis`]: Parsed user input with requirements and ambiguities
//! - [`Question`]: Clarifying questions with multiple types and options
//! - [`ArchitecturePlan`]: Complete architecture specification output
//!
//! ## Engineer Agent
//! - [`EngineerState`]: The 6-state machine (IDLE, ANALYZING_PLAN, GENERATING_CODE, REVIEWING, COMPLETE, ERROR)
//! - [`FileTemplate`]: Specification for a file to be generated
//! - [`GeneratedFile`]: Output file containing generated code
//! - [`CodeQualityCheck`]: Quality validation results
//! - [`CodeOutput`]: Final output bundle with all generated files

// Architect Agent models
pub mod state;
pub mod analysis;
pub mod requirement;
pub mod ambiguity;
pub mod question;
pub mod plan;
pub mod confidence;
pub mod reasoning;

// Engineer Agent models
pub mod engineer_state;
pub mod architecture_plan;
pub mod file_template;
pub mod generated_file;
pub mod quality_check;
pub mod engineer_confidence;
pub mod code_output;

// Architect Agent exports
pub use state::AgentState;
pub use analysis::{SpecificationAnalysis, ProjectIntent};
pub use requirement::{Requirement, RequirementCategory, Priority, Source};
pub use ambiguity::{Ambiguity, Impact};
pub use question::{Question, QuestionType, QuestionOption};
pub use plan::{ArchitecturePlan, ArchitecturePattern, TechStack, Component, ArchitectureDecision};
pub use confidence::ConfidenceBreakdown;
pub use reasoning::{ReasoningEntry, ArchitectPhase, ReasoningType};

// Engineer Agent exports
pub use engineer_state::EngineerState;
pub use architecture_plan::ArchitecturePlan as EngineerArchitecturePlan;
pub use file_template::{FileTemplate, Language};
pub use generated_file::{GeneratedFile, Import, Export, ExportType, TypeDefinition, TypeKind};
pub use quality_check::{CodeQualityCheck, QualityIssue, Severity, CheckCategory, CodeContext};
pub use engineer_confidence::ConfidenceBreakdown as EngineerConfidenceBreakdown;
pub use code_output::{CodeOutput, QualityReport, GenerationMetadata};
