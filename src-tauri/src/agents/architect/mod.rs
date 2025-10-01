// Architect Agent Module
// Analyzes specifications, asks clarifying questions, and designs architecture

pub mod analysis;
pub mod questions;
pub mod design;
pub mod confidence;

// Re-export commonly used items
pub use analysis::{parse_specification, detect_ambiguities, categorize_requirements};
pub use questions::{generate_questions, prioritize_by_impact, combine_related_questions};
pub use design::design_architecture;
pub use confidence::calculate_confidence;
