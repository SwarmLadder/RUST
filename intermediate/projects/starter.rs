/// Read one department,amount per line. Trim both fields, reject blank departments, parse unsigned integer amounts, reject sum overflow. Errors report the one-based source line. Empty input is valid.
pub fn invoices(input: &str) -> Result<std::collections::BTreeMap<String, u64>, usize> {
    todo!("Implement invoices")
}

/// Split on whitespace and apply ASCII lowercasing. Preserve punctuation. List each one-based line number at most once per word. Blank lines still count.
pub fn index(text: &str) -> std::collections::BTreeMap<String, Vec<usize>> {
    todo!("Implement index")
}

/// Use half-open minute ranges in a single day. Reject start >= end or end > 1440 with the first zero-based input index. Sort valid ranges, merging overlapping and touching spans.
pub fn bookings(input: &[(u16, u16)]) -> Result<Vec<(u16, u16)>, usize> {
    todo!("Implement bookings")
}
