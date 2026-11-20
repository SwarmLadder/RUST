/// An empty route has no report. The first sample contributes no ascent. Count only positive changes, widening before subtracting. Return ascent and the zero-based index of the first summit.
pub fn trail(heights: &[i32]) -> Option<(u64, usize)> {
    todo!("Implement trail")
}

/// Each row gives name, on-hand and target quantity. Keep only shortages, emit the missing amount and retain row order. Treat repeated labels as separate shelves.
pub fn shopping<'a>(stock: &[(&'a str, u16, u16)]) -> Vec<(&'a str, u16)> {
    todo!("Implement shopping")
}

/// Return the names attached to the maximum score, preserving input order and duplicate entries. Empty input gives an empty list. Negative scores are valid.
pub fn leaders<'a>(scores: &[(&'a str, i32)]) -> Vec<&'a str> {
    todo!("Implement leaders")
}
