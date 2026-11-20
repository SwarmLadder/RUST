use super::ConfigError;

/// Trim and reject empty setting keys with EmptyKey.
pub fn key(s: &str) -> Result<&str, ConfigError> {
    let s = s.trim();
    if s.is_empty() {
        Err(ConfigError::EmptyKey)
    } else {
        Ok(s)
    }
}

/// Trim and parse a u16 or return InvalidValue. Zero is valid.
pub fn quantity(s: &str) -> Result<u16, ConfigError> {
    s.trim().parse().map_err(|_| ConfigError::InvalidValue)
}

/// Require an equals sign, then a valid key, then a quantity, in that order.
pub fn setting(s: &str) -> Result<(&str, u16), ConfigError> {
    let (k, v) = s.split_once('=').ok_or(ConfigError::MissingEquals)?;
    Ok((key(k)?, quantity(v)?))
}

/// Read every line as a setting. Attach one-based line numbers to errors.
pub fn settings(s: &str) -> Result<Vec<(&str, u16)>, (usize, ConfigError)> {
    s.lines()
        .enumerate()
        .map(|(i, l)| setting(l).map_err(|e| (i + 1, e)))
        .collect()
}

/// Collect every erroneous line and its error in input order.
pub fn all_errors(s: &str) -> Vec<(usize, ConfigError)> {
    s.lines()
        .enumerate()
        .filter_map(|(i, l)| setting(l).err().map(|e| (i + 1, e)))
        .collect()
}
