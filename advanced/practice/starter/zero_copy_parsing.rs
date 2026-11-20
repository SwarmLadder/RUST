/// Skip leading whitespace, take one word, leave the suffix starting at its following whitespace.
pub fn token(s: &str) -> Option<(&str, &str)> {
    todo!("Implement token")
}

/// Read a double-quoted prefix without escapes. Return its contents and unconsumed suffix.
pub fn quoted(s: &str) -> Option<(&str, &str)> {
    todo!("Implement quoted")
}

/// Split at first colon. Require a nonempty ASCII alphabetic key. Trim only the value.
pub fn header(s: &str) -> Option<(&str, &str)> {
    todo!("Implement header")
}

/// Split after exactly n Unicode scalars or fail if too short.
pub fn take_chars(s: &str, n: usize) -> Option<(&str, &str)> {
    todo!("Implement take_chars")
}

/// Return the first valid matching header value using ASCII case-insensitive key comparison. Skip invalid lines.
pub fn find_header<'a>(lines: &'a str, wanted: &str) -> Option<&'a str> {
    todo!("Implement find_header")
}
