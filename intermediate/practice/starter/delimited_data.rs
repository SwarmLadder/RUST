/// Split on tabs preserving empty fields, including the final one.
pub fn cells(s: &str) -> Vec<&str> {
    todo!("Implement cells")
}

/// Prefix vertical bars and backslashes with a backslash.
pub fn escape(s: &str) -> String {
    todo!("Implement escape")
}

/// Split at unescaped vertical bars. Backslash may escape only bar or backslash. Reject dangling escapes.
pub fn decode_row(s: &str) -> Option<Vec<String>> {
    todo!("Implement decode_row")
}

/// Join escaped fields with bars. Reject a row with no fields.
pub fn encode_row(xs: &[&str]) -> Option<String> {
    todo!("Implement encode_row")
}

/// Return column count of a tab-separated table. Empty input has zero columns. Report one-based first unequal line.
pub fn widths(s: &str) -> Result<usize, usize> {
    todo!("Implement widths")
}
