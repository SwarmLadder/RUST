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

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../basics/exercises.rs"]
pub mod basics;
#[cfg(feature = "reference")]
#[path = "../basics/solutions.rs"]
#[doc = include_str!("../basics/workshop-1.md")]
#[doc = include_str!("../basics/workshop-2.md")]
#[doc = include_str!("../basics/workshop-3.md")]
#[doc = include_str!("../basics/workshop-4.md")]
pub mod basics;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../basics/projects/starter.rs"]
pub mod basics_projects;
#[cfg(feature = "reference")]
#[path = "../basics/projects/reference.rs"]
#[doc = include_str!("../basics/projects/trail.md")]
#[doc = include_str!("../basics/projects/shopping.md")]
#[doc = include_str!("../basics/projects/leaders.md")]
pub mod basics_projects;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../intermediate/exercises.rs"]
pub mod intermediate;
#[cfg(feature = "reference")]
#[path = "../intermediate/solutions.rs"]
#[doc = include_str!("../intermediate/workshop-1.md")]
#[doc = include_str!("../intermediate/workshop-2.md")]
#[doc = include_str!("../intermediate/workshop-3.md")]
#[doc = include_str!("../intermediate/workshop-4.md")]
pub mod intermediate;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../intermediate/projects/starter.rs"]
pub mod intermediate_projects;
#[cfg(feature = "reference")]
#[path = "../intermediate/projects/reference.rs"]
#[doc = include_str!("../intermediate/projects/invoices.md")]
#[doc = include_str!("../intermediate/projects/index.md")]
#[doc = include_str!("../intermediate/projects/bookings.md")]
pub mod intermediate_projects;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../advanced/exercises.rs"]
pub mod advanced;
#[cfg(feature = "reference")]
#[path = "../advanced/solutions.rs"]
#[doc = include_str!("../advanced/workshop-1.md")]
#[doc = include_str!("../advanced/workshop-2.md")]
#[doc = include_str!("../advanced/workshop-3.md")]
#[doc = include_str!("../advanced/workshop-4.md")]
pub mod advanced;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "../advanced/projects/starter.rs"]
pub mod advanced_projects;
#[cfg(feature = "reference")]
#[path = "../advanced/projects/reference.rs"]
#[doc = include_str!("../advanced/projects/events.md")]
#[doc = include_str!("../advanced/projects/histogram.md")]
#[doc = include_str!("../advanced/projects/settle.md")]
pub mod advanced_projects;
