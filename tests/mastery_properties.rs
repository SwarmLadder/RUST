//! Deterministic invariant/regression tests that supplement the twelve exercise checks.
//! Starter mode intentionally fails until the learner implements the mastery functions.
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use swarmladder_rust::mastery::*;

#[test]
fn unicode_windows_match_char_slices_across_all_valid_ranges() {
    let text = "aé🙂e\u{301}終";
    let chars: Vec<char> = text.chars().collect();
    for start in 0..=chars.len() {
        for width in 0..=(chars.len() - start) {
            let expected: String = chars[start..start + width].iter().collect();
            assert_eq!(
                unicode_window(text, start, width),
                Some(expected.as_str()),
                "start={start} width={width}"
            );
        }
    }
    assert_eq!(unicode_window(text, chars.len() + 1, 0), None);
}

#[test]
fn mean_is_invariant_under_permutation_and_uniform_values() {
    for value in [-999, -1, 0, 1, 999, i64::MAX, i64::MIN] {
        assert_eq!(mean_truncated(&[value, value, value]), Some(value));
    }
    let a = [i64::MAX, 3, i64::MIN, -2, 1];
    let b = [1, -2, i64::MIN, i64::MAX, 3];
    assert_eq!(mean_truncated(&a), mean_truncated(&b));
}

#[test]
fn normalized_ranges_are_sorted_nonadjacent_and_idempotent() {
    let cases = [
        vec![],
        vec![(0, 0)],
        vec![(5, 8), (1, 1), (2, 4), (7, 9), (11, 11)],
        vec![(u32::MAX, u32::MAX), (0, 0)],
        vec![(3, 3), (1, 1), (3, 3), (2, 2)],
    ];
    for input in cases {
        let output = normalize_ranges(&input).unwrap();
        assert_eq!(normalize_ranges(&output).unwrap(), output);
        for pair in output.windows(2) {
            assert!(pair[0].0 <= pair[0].1);
            assert!(pair[1].0 <= pair[1].1);
            assert!(pair[1].0 > pair[0].1.saturating_add(1));
        }
    }
}

#[test]
fn ranking_of_smaller_k_is_a_prefix_of_full_ranking() {
    let scores: Vec<(String, i64)> = (0..50)
        .map(|i| (format!("name-{}", i % 9), (i as i64 % 7) - 3))
        .collect();
    let full = top_k_scores(&scores, scores.len());
    for k in 0..=scores.len() {
        assert_eq!(top_k_scores(&scores, k), full[..k]);
    }
}

#[test]
fn framed_stream_round_trips_many_small_binary_payloads() {
    let payloads: Vec<Vec<u8>> = (0..10)
        .map(|size| (0..size).map(|n| (n * 31 % 256) as u8).collect())
        .collect();
    let mut encoded = Vec::new();
    for payload in &payloads {
        encoded.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        encoded.extend_from_slice(payload);
    }
    assert_eq!(read_frames(&encoded[..], 10, 10).unwrap(), payloads);
    assert!(read_frames(&encoded[..], 10, 9).is_err());
}

#[test]
fn parallel_histogram_preserves_all_items_for_varied_splits() {
    for size in [0, 1, 2, 17, 256, 1023] {
        let input: Vec<u8> = (0..size).map(|n| (n % 17) as u8).collect();
        let expected = parallel_histogram(&input, 1).unwrap();
        assert_eq!(expected.iter().sum::<usize>(), size);
        for workers in [2, 3, 16, 1000] {
            assert_eq!(parallel_histogram(&input, workers), Some(expected));
        }
    }
}

#[test]
fn transactional_errors_do_not_commit_earlier_successes() {
    let opening = BTreeMap::from([("a".into(), 1u64), ("b".into(), 10u64)]);
    for deltas in [
        vec![("a".into(), 1), ("a".into(), -3)],
        vec![("a".into(), 1), ("missing".into(), 1)],
        vec![("b".into(), 1), ("a".into(), i64::MIN)],
    ] {
        let mut current = opening.clone();
        assert_eq!(apply_changes(&mut current, &deltas), Err(1));
        assert_eq!(current, opening);
    }
}

#[test]
fn reserve_zero_checks_limit_and_overflow_is_non_mutating() {
    let counter = AtomicUsize::new(11);
    assert_eq!(reserve_capacity(&counter, 10, 0), Err(ReservationError::CapacityExceeded));
    assert_eq!(counter.load(Ordering::SeqCst), 11);
    let counter = AtomicUsize::new(usize::MAX - 1);
    assert_eq!(
        reserve_capacity(&counter, usize::MAX, 2),
        Err(ReservationError::CapacityExceeded)
    );
    assert_eq!(counter.load(Ordering::SeqCst), usize::MAX - 1);
}
