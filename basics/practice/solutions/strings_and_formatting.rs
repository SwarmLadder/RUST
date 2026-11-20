/// Trim the name and append a space and # followed by an ID at least four digits wide.
pub fn badge(name: &str, id: u16) -> String {
    format!("{} #{id:04}", name.trim())
}

/// Split at the first colon, trim both sides, preserve empty fields.
pub fn subject(line: &str) -> Option<(&str, &str)> {
    line.split_once(':').map(|(a, b)| (a.trim(), b.trim()))
}

/// Take the first scalar of each whitespace-separated word without changing case.
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .collect()
}

/// Reverse Unicode scalar values.
pub fn reverse_text(s: &str) -> String {
    s.chars().rev().collect()
}

/// Produce one hyphen per Unicode scalar in s.
pub fn underline(s: &str) -> String {
    "-".repeat(s.chars().count())
}
