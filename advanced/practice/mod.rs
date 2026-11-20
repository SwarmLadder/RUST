mod types;
pub use types::*;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/binary_framing.rs"]
pub mod binary_framing;
#[cfg(feature = "reference")]
#[doc = include_str!("binary_framing.md")]
#[path = "solutions/binary_framing.rs"]
pub mod binary_framing;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/graph_traversal.rs"]
pub mod graph_traversal;
#[cfg(feature = "reference")]
#[doc = include_str!("graph_traversal.md")]
#[path = "solutions/graph_traversal.rs"]
pub mod graph_traversal;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/lazy_iterator_construction.rs"]
pub mod lazy_iterator_construction;
#[cfg(feature = "reference")]
#[doc = include_str!("lazy_iterator_construction.md")]
#[path = "solutions/lazy_iterator_construction.rs"]
pub mod lazy_iterator_construction;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/recursive_data_with_box.rs"]
pub mod recursive_data_with_box;
#[cfg(feature = "reference")]
#[doc = include_str!("recursive_data_with_box.md")]
#[path = "solutions/recursive_data_with_box.rs"]
pub mod recursive_data_with_box;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/sharing_and_synchronization.rs"]
pub mod sharing_and_synchronization;
#[cfg(feature = "reference")]
#[doc = include_str!("sharing_and_synchronization.md")]
#[path = "solutions/sharing_and_synchronization.rs"]
pub mod sharing_and_synchronization;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/versioned_state_machines.rs"]
pub mod versioned_state_machines;
#[cfg(feature = "reference")]
#[doc = include_str!("versioned_state_machines.md")]
#[path = "solutions/versioned_state_machines.rs"]
pub mod versioned_state_machines;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/working_with_generic_streams.rs"]
pub mod working_with_generic_streams;
#[cfg(feature = "reference")]
#[doc = include_str!("working_with_generic_streams.md")]
#[path = "solutions/working_with_generic_streams.rs"]
pub mod working_with_generic_streams;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/zero_copy_parsing.rs"]
pub mod zero_copy_parsing;
#[cfg(feature = "reference")]
#[doc = include_str!("zero_copy_parsing.md")]
#[path = "solutions/zero_copy_parsing.rs"]
pub mod zero_copy_parsing;
