/// Given sorted input return the first index with value >= key, or len.
pub fn lower(xs: &[i32], key: i32) -> usize {
    xs.partition_point(|&x| x < key)
}

/// Return the half-open equal range in sorted input, including the insertion position for absent keys.
pub fn equal_range(xs: &[i32], key: i32) -> std::ops::Range<usize> {
    lower(xs, key)..xs.partition_point(|&x| x <= key)
}

/// Sort descending by score, then ascending by name. Return names only.
pub fn rank(rows: &[(String, i32)]) -> Vec<String> {
    let mut v = rows.to_vec();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.into_iter().map(|r| r.0).collect()
}

/// Return the lower median of unsorted input without mutating it.
pub fn median(xs: &[i32]) -> Option<i32> {
    if xs.is_empty() {
        return None;
    }
    let mut v = xs.to_vec();
    let i = (v.len() - 1) / 2;
    Some(*v.select_nth_unstable(i).1)
}

/// Insert into sorted distinct input only if absent and report insertion.
pub fn insert_unique(xs: &mut Vec<i32>, key: i32) -> bool {
    match xs.binary_search(&key) {
        Ok(_) => false,
        Err(i) => {
            xs.insert(i, key);
            true
        }
    }
}
