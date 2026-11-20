/// Yield n down through zero, inclusive, lazily.
pub fn countdown(n: u32) -> impl Iterator<Item = u32> {
    (0..=n).rev()
}

/// Yield every step-th value from index zero. A zero step yields nothing.
pub fn stride(xs: &[i32], step: usize) -> impl Iterator<Item = i32> + '_ {
    xs.iter()
        .copied()
        .step_by(step.max(1))
        .take(if step == 0 { 0 } else { usize::MAX })
}

/// Yield cumulative sums, widening before adding. Stop if the cumulative total overflows u64.
pub fn prefixes(xs: &[u32]) -> impl Iterator<Item = u64> + '_ {
    xs.iter()
        .scan(0u64, |a, &v| {
            *a = a.checked_add(u64::from(v))?;
            Some(*a)
        })
        .fuse()
}

/// Yield 0,1,1,2,… through the final representable u32 Fibonacci value, then terminate.
pub fn fib() -> impl Iterator<Item = u32> {
    let (mut a, mut b) = (Some(0u32), Some(1u32));
    std::iter::from_fn(move || {
        let v = a?;
        a = b;
        b = b.and_then(|n| v.checked_add(n));
        Some(v)
    })
}

/// Yield each consecutive byte and its run length without allocating run storage.
pub fn runs(xs: &[u8]) -> impl Iterator<Item = (u8, usize)> + '_ {
    let mut rest = xs;
    std::iter::from_fn(move || {
        let &v = rest.first()?;
        let n = rest.iter().take_while(|&&x| x == v).count();
        rest = &rest[n..];
        Some((v, n))
    })
}
