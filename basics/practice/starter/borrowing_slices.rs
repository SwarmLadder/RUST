/// Copy the first and last values when both exist.
pub fn ends(xs: &[i32]) -> Option<(i32, i32)> {
    todo!("Implement ends")
}

/// Borrow the final n items, or the entire slice when shorter.
pub fn suffix(xs: &[i32], n: usize) -> &[i32] {
    todo!("Implement suffix")
}

/// Split at n when n is at most the length.
pub fn split_at_checked(xs: &[i32], n: usize) -> Option<(&[i32], &[i32])> {
    todo!("Implement split_at_checked")
}

/// Swap first and last only when there are at least two elements. Report whether a swap occurred.
pub fn swap_ends(xs: &mut [i32]) -> bool {
    todo!("Implement swap_ends")
}

/// Rotate nonempty input left by n positions modulo its length. Empty input stays empty.
pub fn shift(xs: &mut [i32], n: usize) {
    todo!("Implement shift")
}
