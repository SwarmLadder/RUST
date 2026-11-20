/// Move out the current title and leave an empty string.
pub fn take_title(slot: &mut String) -> String {
    std::mem::take(slot)
}

/// Install the incoming title and return the old allocation.
pub fn rename(slot: &mut String, new: String) -> String {
    std::mem::replace(slot, new)
}

/// Remove the final two items in original order. Leave a short list intact.
pub fn pop_pair(xs: &mut Vec<String>) -> Option<(String, String)> {
    if xs.len() < 2 {
        return None;
    }
    let b = xs.pop()?;
    Some((xs.pop()?, b))
}

/// Move nonblank titles to the first vector and whitespace-only ones to the second. Preserve text and order.
pub fn partition_titles(xs: Vec<String>) -> (Vec<String>, Vec<String>) {
    xs.into_iter().partition(|s| !s.trim().is_empty())
}

/// Append a borrowed suffix to the owned prefix and return it.
pub fn join_owned(mut prefix: String, suffix: &str) -> String {
    prefix.push_str(suffix);
    prefix
}
