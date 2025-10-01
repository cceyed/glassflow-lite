// Multi-Agent System Module
// Contains all agent implementations

pub mod architect;
pub mod engineer;
pub mod quality;
pub mod debug;

// Re-export modules for convenience
pub use architect::*;
pub use engineer::Engineer;
pub use quality::Quality;
pub use debug::Debug;
