mod types;
pub use types::*;

#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/borrowing_slices.rs"]
pub mod borrowing_slices;
#[cfg(feature = "reference")]
#[doc = include_str!("borrowing_slices.md")]
#[path = "solutions/borrowing_slices.rs"]
pub mod borrowing_slices;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/decisions_and_ranges.rs"]
pub mod decisions_and_ranges;
#[cfg(feature = "reference")]
#[doc = include_str!("decisions_and_ranges.md")]
#[path = "solutions/decisions_and_ranges.rs"]
pub mod decisions_and_ranges;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/enums_and_matching.rs"]
pub mod enums_and_matching;
#[cfg(feature = "reference")]
#[doc = include_str!("enums_and_matching.md")]
#[path = "solutions/enums_and_matching.rs"]
pub mod enums_and_matching;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/integer_arithmetic.rs"]
pub mod integer_arithmetic;
#[cfg(feature = "reference")]
#[doc = include_str!("integer_arithmetic.md")]
#[path = "solutions/integer_arithmetic.rs"]
pub mod integer_arithmetic;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/loops_and_termination.rs"]
pub mod loops_and_termination;
#[cfg(feature = "reference")]
#[doc = include_str!("loops_and_termination.md")]
#[path = "solutions/loops_and_termination.rs"]
pub mod loops_and_termination;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/small_algorithms.rs"]
pub mod small_algorithms;
#[cfg(feature = "reference")]
#[doc = include_str!("small_algorithms.md")]
#[path = "solutions/small_algorithms.rs"]
pub mod small_algorithms;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/strings_and_formatting.rs"]
pub mod strings_and_formatting;
#[cfg(feature = "reference")]
#[doc = include_str!("strings_and_formatting.md")]
#[path = "solutions/strings_and_formatting.rs"]
pub mod strings_and_formatting;
#[cfg(not(feature = "reference"))]
#[allow(unused_variables, unused_mut)]
#[path = "starter/tuples_and_records.rs"]
pub mod tuples_and_records;
#[cfg(feature = "reference")]
#[doc = include_str!("tuples_and_records.md")]
#[path = "solutions/tuples_and_records.rs"]
pub mod tuples_and_records;
