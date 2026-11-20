/// Copy the first and last values when both exist.
pub fn ends(xs: &[i32]) -> Option<(i32, i32)> {
    Some((*xs.first()?, *xs.last()?))
}

/// Borrow the final n items, or the entire slice when shorter.
pub fn suffix(xs: &[i32], n: usize) -> &[i32] {
    &xs[xs.len().saturating_sub(n)..]
}

/// Split at n when n is at most the length.
pub fn split_at_checked(xs: &[i32], n: usize) -> Option<(&[i32], &[i32])> {
    (n <= xs.len()).then(|| xs.split_at(n))
}

/// Swap first and last only when there are at least two elements. Report whether a swap occurred.
pub fn swap_ends(xs: &mut [i32]) -> bool {
    if xs.len() < 2 {
        return false;
    }
    xs.swap(0, xs.len() - 1);
    true
}

/// Rotate nonempty input left by n positions modulo its length. Empty input stays empty.
pub fn shift(xs: &mut [i32], n: usize) {
    if !xs.is_empty() {
        xs.rotate_left(n % xs.len())
    }
}
