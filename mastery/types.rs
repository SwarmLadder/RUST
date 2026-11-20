//! Shared public error contracts for the mastery labs.
//! These types are unchanged between the starter and reference builds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortError {
    Empty,
    InvalidCharacters,
    OutOfRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingEquals(usize),
    EmptyKey(usize),
    DuplicateKey(usize, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationError {
    CapacityExceeded,
}
