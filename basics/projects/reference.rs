/// An empty route has no report. The first sample contributes no ascent. Count only positive changes, widening before subtracting. Return ascent and the zero-based index of the first summit.
pub fn trail(heights: &[i32]) -> Option<(u64, usize)> {
    let (&first, rest) = heights.split_first()?;
    let mut peak = 0;
    let mut last = first;
    let mut gain = 0;
    for (i, &h) in rest.iter().enumerate() {
        gain += (i64::from(h) - i64::from(last)).max(0) as u64;
        if h > heights[peak] {
            peak = i + 1;
        }
        last = h;
    }
    Some((gain, peak))
}

/// Each row gives name, on-hand and target quantity. Keep only shortages, emit the missing amount and retain row order. Treat repeated labels as separate shelves.
pub fn shopping<'a>(stock: &[(&'a str, u16, u16)]) -> Vec<(&'a str, u16)> {
    stock
        .iter()
        .filter_map(|&(name, have, target)| {
            target
                .checked_sub(have)
                .filter(|&v| v > 0)
                .map(|v| (name, v))
        })
        .collect()
}

/// Return the names attached to the maximum score, preserving input order and duplicate entries. Empty input gives an empty list. Negative scores are valid.
pub fn leaders<'a>(scores: &[(&'a str, i32)]) -> Vec<&'a str> {
    let best = scores.iter().map(|r| r.1).max();
    scores
        .iter()
        .filter(|r| Some(r.1) == best)
        .map(|r| r.0)
        .collect()
}
