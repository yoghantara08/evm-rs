pub mod context;
pub mod database;
pub mod disasm;
pub mod error;
pub mod gas;
pub mod interpreter;
pub mod memory;
pub mod opcode;
pub mod stack;

// Re-export key types for convenience
pub use context::ExecutionContext;
pub use database::{Address, Database, StubDatabase};
pub use error::EvmError;
pub use interpreter::{execute, execute_with_trace, ExecutionResult, Log, TraceStep};
