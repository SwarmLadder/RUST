/// Skip leading whitespace, take one word, leave the suffix starting at its following whitespace.
pub fn token(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        return None;
    }
    let n = s.find(char::is_whitespace).unwrap_or(s.len());
    Some(s.split_at(n))
}

/// Read a double-quoted prefix without escapes. Return its contents and unconsumed suffix.
pub fn quoted(s: &str) -> Option<(&str, &str)> {
    let s = s.strip_prefix('"')?;
    s.split_once('"')
}

/// Split at first colon. Require a nonempty ASCII alphabetic key. Trim only the value.
pub fn header(s: &str) -> Option<(&str, &str)> {
    let (k, v) = s.split_once(':')?;
    (!k.is_empty() && k.bytes().all(|b| b.is_ascii_alphabetic())).then(|| (k, v.trim()))
}

/// Split after exactly n Unicode scalars or fail if too short.
pub fn take_chars(s: &str, n: usize) -> Option<(&str, &str)> {
    let mut i = 0;
    let mut cs = s.chars();
    for _ in 0..n {
        i += cs.next()?.len_utf8()
    }
    Some(s.split_at(i))
}

/// Return the first valid matching header value using ASCII case-insensitive key comparison. Skip invalid lines.
pub fn find_header<'a>(lines: &'a str, wanted: &str) -> Option<&'a str> {
    lines
        .lines()
        .filter_map(header)
        .find(|(k, _)| k.eq_ignore_ascii_case(wanted))
        .map(|(_, v)| v)
}
