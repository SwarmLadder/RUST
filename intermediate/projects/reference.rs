/// Read one department,amount per line. Trim both fields, reject blank departments, parse unsigned integer amounts, reject sum overflow. Errors report the one-based source line. Empty input is valid.
pub fn invoices(input: &str) -> Result<std::collections::BTreeMap<String, u64>, usize> {
    let mut totals = std::collections::BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        let n = i + 1;
        let (dept, value) = line.split_once(',').ok_or(n)?;
        let dept = dept.trim();
        if dept.is_empty() {
            return Err(n);
        }
        let value = value.trim().parse::<u64>().map_err(|_| n)?;
        let entry = totals.entry(dept.to_owned()).or_insert(0u64);
        *entry = entry.checked_add(value).ok_or(n)?;
    }
    Ok(totals)
}

/// Split on whitespace and apply ASCII lowercasing. Preserve punctuation. List each one-based line number at most once per word. Blank lines still count.
pub fn index(text: &str) -> std::collections::BTreeMap<String, Vec<usize>> {
    let mut map = std::collections::BTreeMap::<String, Vec<usize>>::new();
    for (i, line) in text.lines().enumerate() {
        for word in line.split_whitespace() {
            let rows = map.entry(word.to_ascii_lowercase()).or_default();
            if rows.last() != Some(&(i + 1)) {
                rows.push(i + 1)
            }
        }
    }
    map
}

/// Use half-open minute ranges in a single day. Reject start >= end or end > 1440 with the first zero-based input index. Sort valid ranges, merging overlapping and touching spans.
pub fn bookings(input: &[(u16, u16)]) -> Result<Vec<(u16, u16)>, usize> {
    for (i, &(a, b)) in input.iter().enumerate() {
        if a >= b || b > 1440 {
            return Err(i);
        }
    }
    let mut ranges = input.to_vec();
    ranges.sort_unstable();
    let mut out: Vec<(u16, u16)> = Vec::new();
    for (a, b) in ranges {
        if let Some(last) = out.last_mut() {
            if a <= last.1 {
                last.1 = last.1.max(b);
                continue;
            }
        }
        out.push((a, b))
    }
    Ok(out)
}
