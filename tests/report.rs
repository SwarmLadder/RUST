use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use swarmladder_rust::{render, report, sessions, totals, EntryError, Session};

fn row(topic: &str, minutes: u32) -> Session {
    Session { topic: topic.into(), minutes }
}

#[test]
fn lesson5_owns_topics_after_input_is_dropped() {
    let rows = sessions(&String::from("  café ,2\r\n\nRust,0")).unwrap();
    assert_eq!(rows, vec![row("café", 2), row("Rust", 0)]);
}

#[test]
fn lesson5_blank_input_has_no_records() {
    assert!(sessions(" \n\t").unwrap().is_empty());
}

#[test]
fn lesson5_reports_first_error_on_original_line() {
    assert_eq!(sessions("Rust,1\n\nmissing\n,3"), Err((3, EntryError::MissingSeparator)));
}

#[test]
fn lesson6_groups_exact_keys_and_keeps_zero_minutes() {
    let rows = [row("Rust", 20), row("rust", 0), row("Rust", 7)];
    assert_eq!(totals(&rows), BTreeMap::from([("Rust", 27), ("rust", 0)]));
}

#[test]
fn lesson6_widens_before_adding() {
    let rows = [row("A", u32::MAX), row("A", 1)];
    assert_eq!(totals(&rows)["A"], u64::from(u32::MAX) + 1);
}

#[test]
fn lesson6_empty_slice_has_no_keys() {
    assert!(totals(&[]).is_empty());
}

#[test]
fn lesson7_output_uses_key_order_and_final_newline() {
    let map = BTreeMap::from([("Zoo", 3), ("Algebra", 0)]);
    assert_eq!(render(&map), "Algebra: 0 min\nZoo: 3 min\n");
}

#[test]
fn lesson7_empty_report() {
    assert_eq!(render(&BTreeMap::new()), "No sessions.\n");
}

fn cli(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_study-report"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn lesson8_cli_success() {
    let result = cli(include_str!("../fixtures/week.txt"));
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    assert_eq!(result.stdout, b"Ownership: 35 min\nTraits: 15 min\n");
}

#[test]
fn lesson8_cli_failure_writes_only_a_diagnostic() {
    let result = cli("Rust,2\n\nRust,-1");
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert_eq!(result.stderr, b"line 3: minutes must be a nonnegative u32\n");
}

#[test]
fn lesson8_distinguishes_all_input_errors() {
    for (line, reason) in [("x", "expected a comma"), (" ,3", "topic is blank"),
        ("x,NaN", "minutes must be a nonnegative u32")] {
        assert_eq!(report(line), Err(format!("line 1: {reason}")));
    }
}
