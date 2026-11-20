use crate::{EntryError, Summary};

// Lesson 1. Return the number of complete 25-minute blocks and the remainder.
pub fn blocks(_minutes: u32) -> (u32, u32) {
    todo!("split the available minutes into blocks")
}

// Lesson 2. Return the trimmed title, borrowing the original string.
pub fn title(_raw: &str) -> Option<&str> {
    todo!("trim the input and handle blank titles")
}

// Lesson 3. Read a line in the form title,minutes.
pub fn parse_entry(_line: &str) -> Result<(&str, u32), EntryError> {
    todo!("separate the fields and report the first invalid field")
}

// Lesson 4. Skip blank lines, total valid entries and report a bad line by number.
pub fn summarize(_log: &str) -> Result<Summary, (usize, EntryError)> {
    todo!("build a session summary")
}
