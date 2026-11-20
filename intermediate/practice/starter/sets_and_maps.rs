/// Return sorted distinct members of a absent from b.
pub fn missing(a: &[u32], b: &[u32]) -> Vec<u32> {
    todo!("Implement missing")
}

/// Return sorted distinct common IDs.
pub fn common(a: &[u32], b: &[u32]) -> Vec<u32> {
    todo!("Implement common")
}

/// Group names by value, retaining row order inside each bucket.
pub fn invert(rows: &[(&str, u8)]) -> std::collections::BTreeMap<u8, Vec<String>> {
    todo!("Implement invert")
}

/// Return the first ID encountered for the second time.
pub fn duplicate(xs: &[u32]) -> Option<u32> {
    todo!("Implement duplicate")
}

/// Return the most frequent name, resolving ties by smallest lexicographic name.
pub fn vote(rows: &[&str]) -> Option<String> {
    todo!("Implement vote")
}
