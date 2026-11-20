//! Run the completed extension with:
//! cargo run --example mastery_demo --features reference
//! Running without 'reference' intentionally exercises unfinished starter functions.
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use swarmladder_rust::mastery::{
    apply_changes, normalize_ranges, parallel_histogram, parse_port, parse_settings,
    read_frames, reserve_capacity, unicode_window, word_frequency,
};

fn main() {
    let port = parse_port(" 8080 ").expect("valid port");
    let window = unicode_window("Aé🙂Z", 1, 2).expect("valid scalar window");
    let words = word_frequency("Rust! rust");
    let config = parse_settings("mode=safe").expect("valid configuration");
    let ranges = normalize_ranges(&[(3, 4), (1, 2)]).expect("valid ranges");
    let frames = read_frames(&b"\x00\x01x"[..], 3, 2).expect("valid byte frame");
    let histogram = parallel_histogram(b"aba", 2).expect("nonzero workers");
    let mut balances = BTreeMap::from([("course".to_owned(), 5u64)]);
    apply_changes(&mut balances, &[("course".to_owned(), 2)]).expect("valid transaction");
    let seats = AtomicUsize::new(1);
    reserve_capacity(&seats, 3, 1).expect("available capacity");

    assert_eq!(config["mode"], "safe");
    assert_eq!(ranges, vec![(1, 4)]);
    println!(
        "Rust mastery OK: port={port}; window={window}; words={}; frames={}; count_a={}; balance={}; seats={}",
        words["rust"],
        frames.len(),
        histogram[usize::from(b'a')],
        balances["course"],
        seats.load(Ordering::SeqCst),
    );
}
