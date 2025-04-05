// Re-export modules for use in other crates

pub mod auth;
pub mod api;
pub mod db;
pub mod speech;
pub mod ai;
pub mod frontend;
pub mod config;
pub mod error;

// Export common types for convenience
pub use error::AppError;
pub use config::Config;