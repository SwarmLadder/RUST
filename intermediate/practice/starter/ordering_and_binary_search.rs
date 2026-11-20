/// Given sorted input return the first index with value >= key, or len.
pub fn lower(xs: &[i32], key: i32) -> usize {
    todo!("Implement lower")
}

/// Return the half-open equal range in sorted input, including the insertion position for absent keys.
pub fn equal_range(xs: &[i32], key: i32) -> std::ops::Range<usize> {
    todo!("Implement equal_range")
}

/// Sort descending by score, then ascending by name. Return names only.
pub fn rank(rows: &[(String, i32)]) -> Vec<String> {
    todo!("Implement rank")
}

/// Return the lower median of unsorted input without mutating it.
pub fn median(xs: &[i32]) -> Option<i32> {
    todo!("Implement median")
}

/// Insert into sorted distinct input only if absent and report insertion.
pub fn insert_unique(xs: &mut Vec<i32>, key: i32) -> bool {
    todo!("Implement insert_unique")
}
