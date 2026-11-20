//! Acceptance checks: default starter mode has intentional todo! panics.
//! Validate the answer key with: cargo test --features reference --test mastery
use std::collections::BTreeMap;
use std::io::{Cursor, ErrorKind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use swarmladder_rust::mastery::*;

#[test]
fn b01_ports_validate_format_and_bounds() {
    assert_eq!(parse_port(" 443 "), Ok(443));
    assert_eq!(parse_port("00001"), Ok(1));
    assert_eq!(parse_port("65535"), Ok(65535));
    assert_eq!(parse_port(" \t "), Err(PortError::Empty));
    assert_eq!(parse_port("0"), Err(PortError::OutOfRange));
    assert_eq!(parse_port("65536"), Err(PortError::OutOfRange));
    for bad in ["-1", "+20", "3.5", "１２"] {
        assert_eq!(parse_port(bad), Err(PortError::InvalidCharacters));
    }
}

#[test]
fn b02_utf8_character_slices_are_safe() {
    assert_eq!(unicode_window("Aé🙂Z", 1, 2), Some("é🙂"));
    assert_eq!(unicode_window("Aé🙂Z", 4, 0), Some(""));
    assert_eq!(unicode_window("Aé🙂Z", 0, 0), Some(""));
    assert_eq!(unicode_window("Aé🙂Z", 3, 1), Some("Z"));
    assert_eq!(unicode_window("", 0, 0), Some(""));
    assert_eq!(unicode_window("é", 0, 1), Some("é"));
    assert_eq!(unicode_window("Aé🙂Z", 5, 0), None);
    assert_eq!(unicode_window("Aé🙂Z", 2, 4), None);
    assert_eq!(unicode_window("Aé🙂Z", usize::MAX, 1), None);
}

#[test]
fn b03_means_use_wide_arithmetic() {
    assert_eq!(mean_truncated(&[]), None);
    assert_eq!(mean_truncated(&[5]), Some(5));
    assert_eq!(mean_truncated(&[1, 2]), Some(1));
    assert_eq!(mean_truncated(&[-1, -2]), Some(-1)); // toward zero
    assert_eq!(mean_truncated(&[-7, 1, 1]), Some(-1));
    assert_eq!(mean_truncated(&[i64::MAX, i64::MAX]), Some(i64::MAX));
    assert_eq!(mean_truncated(&[i64::MIN, i64::MIN]), Some(i64::MIN));
    assert_eq!(mean_truncated(&[i64::MAX, i64::MIN]), Some(0));
}

#[test]
fn b04_words_normalize_but_preserve_internal_punctuation() {
    let counts = word_frequency("RUST rust. Rust!  É é  co-op ... !!!");
    assert_eq!(counts.get("rust"), Some(&3));
    assert_eq!(counts.get("é"), Some(&2));
    assert_eq!(counts.get("co-op"), Some(&1));
    assert_eq!(counts.len(), 3);
    assert_eq!(word_frequency(" \t... \n"), BTreeMap::new());
    assert_eq!(word_frequency("abc,def abc,def").get("abc,def"), Some(&2));
}

#[test]
fn i01_settings_report_first_physical_line_and_duplicate() {
    let expected = BTreeMap::from([
        ("mode".to_owned(), "safe".to_owned()),
        ("message".to_owned(), "hello=world".to_owned()),
        ("empty".to_owned(), "".to_owned()),
    ]);
    assert_eq!(parse_settings("# comment\nmode = safe\n\nmessage=hello=world\nempty="), Ok(expected));
    assert_eq!(parse_settings("x"), Err(ConfigError::MissingEquals(1)));
    assert_eq!(parse_settings("\n #hi\n=wrong"), Err(ConfigError::EmptyKey(3)));
    assert_eq!(parse_settings("x=one\n#skip\nx=two"), Err(ConfigError::DuplicateKey(3, "x".into())));
    assert_eq!(parse_settings("x=1\nbroken\nx=2"), Err(ConfigError::MissingEquals(2)));
    assert!(parse_settings("A=1\na=2").is_ok());
}

#[test]
fn i02_borrowed_view_preserves_order_and_unicode() {
    let words = vec![
        "  apple ".to_owned(), " apricot".to_owned(), "éclair".to_owned(),
        "  ".to_owned(), "banana".to_owned(), "Älpha".to_owned()
    ];
    let grouped = by_initial(&words);
    assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
    assert_eq!(grouped.get(&'é'), Some(&vec!["éclair"]));
    assert_eq!(grouped.get(&'Ä'), Some(&vec!["Älpha"]));
    assert!(!grouped.contains_key(&' '));
    assert_eq!(grouped.values().map(Vec::len).sum::<usize>(), 5);
    assert!(by_initial(&[]).is_empty());
}

#[test]
fn i03_intervals_sort_merge_and_reject_overflow_traps() {
    assert_eq!(normalize_ranges(&[(5, 7), (1, 2), (3, 4), (10, 10)]), Ok(vec![(1, 7), (10, 10)]));
    assert_eq!(normalize_ranges(&[(5, 5), (5, 9), (8, 12)]), Ok(vec![(5, 12)]));
    assert_eq!(
        normalize_ranges(&[(u32::MAX, u32::MAX), (u32::MAX - 1, u32::MAX - 1)]),
        Ok(vec![(u32::MAX - 1, u32::MAX)])
    );
    assert_eq!(normalize_ranges(&[(1, 2), (9, 8)]), Err(1));
    assert_eq!(normalize_ranges(&[]), Ok(vec![]));
}

#[test]
fn i04_ranking_is_deterministic_on_ties() {
    let entries = vec![
        ("Zed".into(), 4), ("Ada".into(), 10), ("Bob".into(), 10),
        ("Ana".into(), 10), ("Ada".into(), 10), ("Min".into(), -5),
    ];
    assert_eq!(top_k_scores(&entries, 0), vec![]);
    assert_eq!(
        top_k_scores(&entries, 4),
        vec![("Ada".into(), 10), ("Ada".into(), 10), ("Ana".into(), 10), ("Bob".into(), 10)]
    );
    assert_eq!(top_k_scores(&entries, 100).len(), entries.len());
    assert_eq!(top_k_scores(&[("x".into(), i64::MIN)], 1), vec![("x".into(), i64::MIN)]);
}

#[test]
fn a01_frames_detect_truncation_limits_and_eof() {
    let bytes = [0, 0, 0, 2, 0x11, 0xff, 0, 1, 0x41];
    assert_eq!(read_frames(Cursor::new(bytes), 2, 3).unwrap(), vec![vec![], vec![0x11, 0xff], vec![0x41]]);
    assert_eq!(read_frames(&b""[..], 0, 0).unwrap(), Vec::<Vec<u8>>::new());
    assert_eq!(read_frames(&b"\x00"[..], 9, 5).unwrap_err().kind(), ErrorKind::UnexpectedEof);
    assert_eq!(read_frames(&b"\x00\x02x"[..], 9, 5).unwrap_err().kind(), ErrorKind::UnexpectedEof);
    assert_eq!(read_frames(&b"\x00\x02xy"[..], 1, 5).unwrap_err().kind(), ErrorKind::InvalidData);
    assert_eq!(read_frames(&b"\x00\x00"[..], 0, 0).unwrap_err().kind(), ErrorKind::InvalidData);
    assert_eq!(read_frames(&b"\x00\x00\x00\x00"[..], 0, 1).unwrap_err().kind(), ErrorKind::InvalidData);
}

#[test]
fn a02_parallel_histogram_handles_any_worker_budget() {
    assert_eq!(parallel_histogram(&[], 1), Some([0usize; 256]));
    assert_eq!(parallel_histogram(&[], 0), None);
    assert_eq!(parallel_histogram(&[1, 1, 2], 0), None);
    let input: Vec<u8> = (0..5000).map(|i| (i % 256) as u8).collect();
    let serial = parallel_histogram(&input, 1).unwrap();
    assert_eq!(parallel_histogram(&input, 2), Some(serial));
    assert_eq!(parallel_histogram(&input, 100_000), Some(serial));
    assert_eq!(serial.iter().sum::<usize>(), input.len());
    assert_eq!(serial[0], 20);
    assert_eq!(serial[255], 19);
}

#[test]
fn a03_atomic_map_update_rolls_back_on_error() {
    let mut balances = BTreeMap::from([
        ("Ada".into(), 6u64), ("Bob".into(), u64::MAX - 1),
    ]);
    assert_eq!(apply_changes(&mut balances, &[("Ada".into(), -2), ("Ada".into(), 5)]), Ok(()));
    assert_eq!(balances["Ada"], 9);
    assert_eq!(apply_changes(&mut balances, &[("Ada".into(), -10)]), Err(0));
    assert_eq!(balances["Ada"], 9);
    assert_eq!(
        apply_changes(&mut balances, &[("Ada".into(), -2), ("Missing".into(), 1)]),
        Err(1)
    );
    assert_eq!(balances["Ada"], 9);
    assert_eq!(apply_changes(&mut balances, &[("Bob".into(), 3)]), Err(0));
    assert_eq!(balances["Bob"], u64::MAX - 1);
    assert_eq!(apply_changes(&mut balances, &[]), Ok(()));
}

#[test]
fn a04_atomic_capacity_is_bounded_even_under_races() {
    let count = AtomicUsize::new(5);
    assert_eq!(reserve_capacity(&count, 10, 3), Ok(5));
    assert_eq!(count.load(Ordering::Relaxed), 8);
    assert_eq!(reserve_capacity(&count, 10, 3), Err(ReservationError::CapacityExceeded));
    assert_eq!(reserve_capacity(&count, 10, 0), Ok(8));
    assert_eq!(count.load(Ordering::Relaxed), 8);
    let full = AtomicUsize::new(usize::MAX);
    assert_eq!(reserve_capacity(&full, usize::MAX, 1), Err(ReservationError::CapacityExceeded));
    let shared = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for _ in 0..64 {
        let counter = Arc::clone(&shared);
        handles.push(std::thread::spawn(move || reserve_capacity(&counter, 20, 1).is_ok()));
    }
    let successes = handles.into_iter().map(|j| j.join().unwrap()).filter(|ok| *ok).count();
    assert_eq!(successes, 20);
    assert_eq!(shared.load(Ordering::SeqCst), 20);
}
