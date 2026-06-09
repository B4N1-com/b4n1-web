//! Session manager module
//!
//! Handles lightweight, tab-based browser sessions.

pub mod server;
pub mod types;

pub use server::*;
pub use types::{Session, SessionKind, State};
