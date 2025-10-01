// IPC Module
// Tauri command handlers for frontend-backend communication

pub mod commands;
pub mod engineer_commands;
pub mod quality_commands;
pub mod debug_commands;
pub mod orchestrator_commands;

pub use commands::*;
pub use engineer_commands::*;
pub use quality_commands::*;
pub use debug_commands::*;
pub use orchestrator_commands::*;
