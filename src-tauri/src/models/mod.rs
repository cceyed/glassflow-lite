//! Data Models Module
//!
//! Core data structures for the Architect Agent system.
//!
//! This module contains all the data models used throughout the Architect Agent,
//! including state management, specification analysis, questions, architecture plans,
//! and confidence scoring.
//!
//! # Key Components
//!
//! - [`AgentState`]: The 6-state machine (IDLE, ANALYZING, QUESTIONING, DESIGNING, COMPLETE, ERROR)
//! - [`SpecificationAnalysis`]: Parsed user input with requirements and ambiguities
//! - [`Question`]: Clarifying questions with multiple types and options
//! - [`ArchitecturePlan`]: Complete architecture specification output
//! - [`ConfidenceBreakdown`]: Detailed confidence metrics with weighted formula

pub mod state;
pub mod analysis;
pub mod requirement;
pub mod ambiguity;
pub mod question;
pub mod plan;
pub mod confidence;
pub mod reasoning;

pub use state::AgentState;
pub use analysis::{SpecificationAnalysis, ProjectIntent};
pub use requirement::{Requirement, RequirementCategory, Priority, Source};
pub use ambiguity::{Ambiguity, Impact};
pub use question::{Question, QuestionType, QuestionOption};
pub use plan::{ArchitecturePlan, ArchitecturePattern, TechStack, Component, ArchitectureDecision};
pub use confidence::ConfidenceBreakdown;
pub use reasoning::{ReasoningEntry, ArchitectPhase, ReasoningType};
