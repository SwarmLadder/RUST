/// Accept nondecreasing input, including empty slices.
pub fn sorted(xs: &[i32]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

/// Find the longest consecutive run of key.
pub fn streak(xs: &[i32], key: i32) -> usize {
    let (mut best, mut run) = (0, 0);
    for &v in xs {
        if v == key {
            run += 1;
            best = best.max(run)
        } else {
            run = 0
        }
    }
    best
}

/// Count adjacent unequal pairs.
pub fn changes(xs: &[i32]) -> usize {
    xs.windows(2).filter(|w| w[0] != w[1]).count()
}

/// Check scalar symmetry, including spaces and case.
pub fn palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

/// Interleave elements starting with a, then append the longer remainder.
pub fn merge_pairs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for i in 0..a.len().max(b.len()) {
        out.extend(a.get(i));
        out.extend(b.get(i));
    }
    out
}
