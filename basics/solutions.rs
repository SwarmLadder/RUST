/// Return cold below zero, otherwise mild.
pub fn temperature(t: i32) -> &'static str {
    if t < 0 {
        "cold"
    } else {
        "mild"
    }
}

/// Count readings at or below zero.
pub fn freezing(values: &[i32]) -> usize {
    values.iter().filter(|&&t| t <= 0).count()
}

/// Increase a counter by one, stopping at 255.
pub fn tick(value: u8) -> u8 {
    value.saturating_add(1)
}

/// Trim surrounding whitespace and borrow the remaining text.
pub fn label(s: &str) -> &str {
    s.trim()
}

/// Count Unicode scalar values.
pub fn scalar_count(s: &str) -> usize {
    s.chars().count()
}

/// Return the first scalar after trimming, or None.
pub fn initial(s: &str) -> Option<char> {
    s.trim().chars().next()
}

/// Find the first even value in input order.
pub fn first_even(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().find(|v| v % 2 == 0)
}

/// Return the largest reading, including negative values.
pub fn peak(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().max()
}

/// Return the first matching zero-based index.
pub fn position_of(xs: &[i32], key: i32) -> Option<usize> {
    xs.iter().position(|&v| v == key)
}

/// Keep strictly positive readings in order.
pub fn positives(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().filter(|&v| v > 0).collect()
}

/// Return scalar counts in the original order.
pub fn lengths(words: &[&str]) -> Vec<usize> {
    words.iter().map(|s| s.chars().count()).collect()
}

/// Return adjacent overlapping pairs.
pub fn pairs(xs: &[i32]) -> Vec<(i32, i32)> {
    xs.windows(2).map(|w| (w[0], w[1])).collect()
}
