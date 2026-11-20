mod types;
pub use types::*;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/closures_and_policies.rs"]
pub mod closures_and_policies;
#[cfg(feature = "reference")]
#[doc = include_str!("closures_and_policies.md")]
#[path = "solutions/closures_and_policies.rs"]
pub mod closures_and_policies;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/delimited_data.rs"]
pub mod delimited_data;
#[cfg(feature = "reference")]
#[doc = include_str!("delimited_data.md")]
#[path = "solutions/delimited_data.rs"]
pub mod delimited_data;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/domain_errors.rs"]
pub mod domain_errors;
#[cfg(feature = "reference")]
#[doc = include_str!("domain_errors.md")]
#[path = "solutions/domain_errors.rs"]
pub mod domain_errors;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/ordering_and_binary_search.rs"]
pub mod ordering_and_binary_search;
#[cfg(feature = "reference")]
#[doc = include_str!("ordering_and_binary_search.md")]
#[path = "solutions/ordering_and_binary_search.rs"]
pub mod ordering_and_binary_search;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/ownership_and_mutation.rs"]
pub mod ownership_and_mutation;
#[cfg(feature = "reference")]
#[doc = include_str!("ownership_and_mutation.md")]
#[path = "solutions/ownership_and_mutation.rs"]
pub mod ownership_and_mutation;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/sets_and_maps.rs"]
pub mod sets_and_maps;
#[cfg(feature = "reference")]
#[doc = include_str!("sets_and_maps.md")]
#[path = "solutions/sets_and_maps.rs"]
pub mod sets_and_maps;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/stacks_and_queues.rs"]
pub mod stacks_and_queues;
#[cfg(feature = "reference")]
#[doc = include_str!("stacks_and_queues.md")]
#[path = "solutions/stacks_and_queues.rs"]
pub mod stacks_and_queues;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/trait_based_interfaces.rs"]
pub mod trait_based_interfaces;
#[cfg(feature = "reference")]
#[doc = include_str!("trait_based_interfaces.md")]
#[path = "solutions/trait_based_interfaces.rs"]
pub mod trait_based_interfaces;
