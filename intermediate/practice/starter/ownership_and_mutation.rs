/// Move out the current title and leave an empty string.
pub fn take_title(slot: &mut String) -> String {
    todo!("Implement take_title")
}

/// Install the incoming title and return the old allocation.
pub fn rename(slot: &mut String, new: String) -> String {
    todo!("Implement rename")
}

/// Remove the final two items in original order. Leave a short list intact.
pub fn pop_pair(xs: &mut Vec<String>) -> Option<(String, String)> {
    todo!("Implement pop_pair")
}

/// Move nonblank titles to the first vector and whitespace-only ones to the second. Preserve text and order.
pub fn partition_titles(xs: Vec<String>) -> (Vec<String>, Vec<String>) {
    todo!("Implement partition_titles")
}

/// Append a borrowed suffix to the owned prefix and return it.
pub fn join_owned(mut prefix: String, suffix: &str) -> String {
    todo!("Implement join_owned")
}
