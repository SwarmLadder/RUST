/// Yield n down through zero, inclusive, lazily.
pub fn countdown(n: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(|| todo!("Implement countdown"))
}

/// Yield every step-th value from index zero. A zero step yields nothing.
pub fn stride(xs: &[i32], step: usize) -> impl Iterator<Item = i32> + '_ {
    std::iter::from_fn(|| todo!("Implement stride"))
}

/// Yield cumulative sums, widening before adding. Stop if the cumulative total overflows u64.
pub fn prefixes(xs: &[u32]) -> impl Iterator<Item = u64> + '_ {
    std::iter::from_fn(|| todo!("Implement prefixes"))
}

/// Yield 0,1,1,2,… through the final representable u32 Fibonacci value, then terminate.
pub fn fib() -> impl Iterator<Item = u32> {
    std::iter::from_fn(|| todo!("Implement fib"))
}

/// Yield each consecutive byte and its run length without allocating run storage.
pub fn runs(xs: &[u8]) -> impl Iterator<Item = (u8, usize)> + '_ {
    std::iter::from_fn(|| todo!("Implement runs"))
}
