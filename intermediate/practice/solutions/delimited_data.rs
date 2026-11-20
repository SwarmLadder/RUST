/// Split on tabs preserving empty fields, including the final one.
pub fn cells(s: &str) -> Vec<&str> {
    s.split('\t').collect()
}

/// Prefix vertical bars and backslashes with a backslash.
pub fn escape(s: &str) -> String {
    let mut o = String::new();
    for c in s.chars() {
        if c == '|' || c == '\\' {
            o.push('\\')
        }
        o.push(c)
    }
    o
}

/// Split at unescaped vertical bars. Backslash may escape only bar or backslash. Reject dangling escapes.
pub fn decode_row(s: &str) -> Option<Vec<String>> {
    let mut out = vec![];
    let mut word = String::new();
    let mut cs = s.chars();
    while let Some(c) = cs.next() {
        match c {
            '|' => out.push(std::mem::take(&mut word)),
            '\\' => {
                let n = cs.next()?;
                if n != '|' && n != '\\' {
                    return None;
                }
                word.push(n)
            }
            _ => word.push(c),
        }
    }
    out.push(word);
    Some(out)
}

/// Join escaped fields with bars. Reject a row with no fields.
pub fn encode_row(xs: &[&str]) -> Option<String> {
    (!xs.is_empty()).then(|| xs.iter().map(|s| escape(s)).collect::<Vec<_>>().join("|"))
}

/// Return column count of a tab-separated table. Empty input has zero columns. Report one-based first unequal line.
pub fn widths(s: &str) -> Result<usize, usize> {
    let mut lines = s.lines();
    let Some(first) = lines.next() else {
        return Ok(0);
    };
    let n = cells(first).len();
    for (i, l) in lines.enumerate() {
        if cells(l).len() != n {
            return Err(i + 2);
        }
    }
    Ok(n)
}
