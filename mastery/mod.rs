#![doc = include_str!("README.md")]

mod types;
pub use types::*;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables)]
#[path = "starter.rs"]
mod implementation;

#[cfg(feature = "reference")]
#[path = "reference.rs"]
mod implementation;

pub use implementation::*;
