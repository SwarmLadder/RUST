use swarmladder_rust::{blocks, parse_entry, summarize, title, EntryError, Summary};

#[test]
fn lesson1_zero() { assert_eq!(blocks(0), (0, 0)); }
#[test]
fn lesson1_exact_block() { assert_eq!(blocks(50), (2, 0)); }
#[test]
fn lesson1_remainder() { assert_eq!(blocks(64), (2, 14)); }

#[test]
fn lesson2_blank() { assert_eq!(title(" \t "), None); }
#[test]
fn lesson2_inner_spaces() { assert_eq!(title("  Rust basics "), Some("Rust basics")); }
#[test]
fn lesson2_unicode() { assert_eq!(title("  café  "), Some("café")); }

#[test]
fn lesson3_valid() { assert_eq!(parse_entry("  Study , 0 "), Ok(("Study", 0))); }
#[test]
fn lesson3_missing_separator() { assert_eq!(parse_entry("Rust"), Err(EntryError::MissingSeparator)); }
#[test]
fn lesson3_title_first() { assert_eq!(parse_entry(" ,bad"), Err(EntryError::EmptyTitle)); }
#[test]
fn lesson3_invalid_minutes() {
    for entry in ["A,-1", "A,3.5", "A,", "A,4294967296", "A,3,4"] {
        assert_eq!(parse_entry(entry), Err(EntryError::InvalidMinutes));
    }
}

#[test]
fn lesson4_empty() { assert_eq!(summarize(" \n"), Ok(Summary { sessions: 0, minutes: 0 })); }
#[test]
fn lesson4_wide_total() {
    assert_eq!(summarize("A,4294967295\r\n \r\nB,4"), Ok(Summary { sessions: 2, minutes: 4294967299 }));
}
#[test]
fn lesson4_physical_line_number() {
    assert_eq!(summarize("A,25\n\nB,ten\ninvalid"), Err((3, EntryError::InvalidMinutes)));
}
