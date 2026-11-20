//! Practice functions for the Rust foundations course.
#[cfg(not(feature = "reference"))]
mod exercises;
#[cfg(not(feature = "reference"))]
pub use exercises::*;

#[cfg(feature = "reference")]
#[path = "../solutions/reference.rs"]
mod reference;
#[cfg(feature = "reference")]
pub use reference::*;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryError {
    MissingSeparator,
    EmptyTitle,
    InvalidMinutes,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub sessions: usize,
    pub minutes: u64,
}

/// An owned record that can outlive the text it was parsed from.
#[derive(Debug, PartialEq, Eq)]
pub struct Session {
    pub topic: String,
    pub minutes: u32,
}
