use super::ConfigError;

/// Trim and reject empty setting keys with EmptyKey.
pub fn key(s: &str) -> Result<&str, ConfigError> {
    todo!("Implement key")
}

/// Trim and parse a u16 or return InvalidValue. Zero is valid.
pub fn quantity(s: &str) -> Result<u16, ConfigError> {
    todo!("Implement quantity")
}

/// Require an equals sign, then a valid key, then a quantity, in that order.
pub fn setting(s: &str) -> Result<(&str, u16), ConfigError> {
    todo!("Implement setting")
}

/// Read every line as a setting. Attach one-based line numbers to errors.
pub fn settings(s: &str) -> Result<Vec<(&str, u16)>, (usize, ConfigError)> {
    todo!("Implement settings")
}

/// Collect every erroneous line and its error in input order.
pub fn all_errors(s: &str) -> Vec<(usize, ConfigError)> {
    todo!("Implement all_errors")
}
