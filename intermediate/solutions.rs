/// Trim and parse a port. Return 'invalid' for a non-u16 and 'zero' for port 0.
pub fn port(raw: &str) -> Result<u16, &'static str> {
    let n = raw.trim().parse::<u16>().map_err(|_| "invalid")?;
    if n == 0 {
        Err("zero")
    } else {
        Ok(n)
    }
}

/// Borrow the trimmed value or return 'empty'.
pub fn required(s: &str) -> Result<&str, &'static str> {
    let s = s.trim();
    if s.is_empty() {
        Err("empty")
    } else {
        Ok(s)
    }
}

/// Parse every trimmed line. Return the first invalid physical line number, starting at one.
pub fn numbers(lines: &str) -> Result<Vec<i32>, usize> {
    lines
        .lines()
        .enumerate()
        .map(|(i, s)| s.trim().parse().map_err(|_| i + 1))
        .collect()
}

/// Count exact whitespace-delimited words with borrowed keys.
pub fn word_counts(text: &str) -> std::collections::BTreeMap<&str, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }
    counts
}

/// Keep one copy of each value in increasing order.
pub fn distinct(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Return the earliest word in input order whose total frequency is one.
pub fn first_unique(text: &str) -> Option<&str> {
    let counts = word_counts(text);
    text.split_whitespace().find(|w| counts[w] == 1)
}

/// Borrow the larger value, choosing the first on a tie.
pub fn larger<'a, T: Ord>(a: &'a T, b: &'a T) -> &'a T {
    if a >= b {
        a
    } else {
        b
    }
}

/// Count matches using a stateful predicate.
pub fn tally<T>(xs: &[T], mut accepts: impl FnMut(&T) -> bool) -> usize {
    xs.iter().filter(|x| accepts(x)).count()
}

/// Return the last match. Visit items from the start, once each.
pub fn last_match<T>(xs: &[T], mut accepts: impl FnMut(&T) -> bool) -> Option<&T> {
    let mut last = None;
    for x in xs {
        if accepts(x) {
            last = Some(x);
        }
    }
    last
}

/// Keep successfully parsed, trimmed u32 values. Preserve duplicates.
pub fn valid_ids(raw: &[&str]) -> Vec<u32> {
    raw.iter().filter_map(|s| s.trim().parse().ok()).collect()
}

/// Return each cumulative sum, widening before addition.
pub fn running(xs: &[i32]) -> Vec<i64> {
    xs.iter()
        .scan(0i64, |sum, &x| {
            *sum += i64::from(x);
            Some(*sum)
        })
        .collect()
}

/// Compute a dot product for equal lengths. Return None on mismatch.
pub fn dot(a: &[i16], b: &[i16]) -> Option<i64> {
    if a.len() != b.len() {
        return None;
    }
    Some(
        a.iter()
            .zip(b)
            .map(|(&x, &y)| i64::from(x) * i64::from(y))
            .sum(),
    )
}
