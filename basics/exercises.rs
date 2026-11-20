/// Return cold below zero, otherwise mild.
pub fn temperature(t: i32) -> &'static str {
    todo!("Implement temperature")
}

/// Count readings at or below zero.
pub fn freezing(values: &[i32]) -> usize {
    todo!("Implement freezing")
}

/// Increase a counter by one, stopping at 255.
pub fn tick(value: u8) -> u8 {
    todo!("Implement tick")
}

/// Trim surrounding whitespace and borrow the remaining text.
pub fn label(s: &str) -> &str {
    todo!("Implement label")
}

/// Count Unicode scalar values.
pub fn scalar_count(s: &str) -> usize {
    todo!("Implement scalar_count")
}

/// Return the first scalar after trimming, or None.
pub fn initial(s: &str) -> Option<char> {
    todo!("Implement initial")
}

/// Find the first even value in input order.
pub fn first_even(xs: &[i32]) -> Option<i32> {
    todo!("Implement first_even")
}

/// Return the largest reading, including negative values.
pub fn peak(xs: &[i32]) -> Option<i32> {
    todo!("Implement peak")
}

/// Return the first matching zero-based index.
pub fn position_of(xs: &[i32], key: i32) -> Option<usize> {
    todo!("Implement position_of")
}

/// Keep strictly positive readings in order.
pub fn positives(xs: &[i32]) -> Vec<i32> {
    todo!("Implement positives")
}

/// Return scalar counts in the original order.
pub fn lengths(words: &[&str]) -> Vec<usize> {
    todo!("Implement lengths")
}

/// Return adjacent overlapping pairs.
pub fn pairs(xs: &[i32]) -> Vec<(i32, i32)> {
    todo!("Implement pairs")
}
