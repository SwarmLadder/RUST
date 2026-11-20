/// Return sorted distinct members of a absent from b.
pub fn missing(a: &[u32], b: &[u32]) -> Vec<u32> {
    let a: std::collections::BTreeSet<_> = a.iter().copied().collect();
    let b = b.iter().copied().collect();
    a.difference(&b).copied().collect()
}

/// Return sorted distinct common IDs.
pub fn common(a: &[u32], b: &[u32]) -> Vec<u32> {
    let a: std::collections::BTreeSet<_> = a.iter().copied().collect();
    let b = b.iter().copied().collect();
    a.intersection(&b).copied().collect()
}

/// Group names by value, retaining row order inside each bucket.
pub fn invert(rows: &[(&str, u8)]) -> std::collections::BTreeMap<u8, Vec<String>> {
    let mut m = std::collections::BTreeMap::<_, Vec<_>>::new();
    for &(k, v) in rows {
        m.entry(v).or_default().push(k.to_owned())
    }
    m
}

/// Return the first ID encountered for the second time.
pub fn duplicate(xs: &[u32]) -> Option<u32> {
    let mut seen = std::collections::HashSet::new();
    xs.iter().copied().find(|v| !seen.insert(*v))
}

/// Return the most frequent name, resolving ties by smallest lexicographic name.
pub fn vote(rows: &[&str]) -> Option<String> {
    let mut m = std::collections::BTreeMap::new();
    for &r in rows {
        *m.entry(r).or_insert(0usize) += 1
    }
    m.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(a.0)))
        .map(|(k, _)| k.to_owned())
}
