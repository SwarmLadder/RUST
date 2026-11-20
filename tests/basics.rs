#[test]
fn temperature() {
    use swarmladder_rust::basics::temperature;
    assert_eq!(temperature(-1), "cold");
    assert_eq!(temperature(0), "mild");
}

#[test]
fn freezing() {
    use swarmladder_rust::basics::freezing;
    assert_eq!(freezing(&[-1, 0, 1]), 2);
    assert_eq!(freezing(&[]), 0);
}

#[test]
fn tick() {
    use swarmladder_rust::basics::tick;
    assert_eq!(tick(254), 255);
    assert_eq!(tick(255), 255);
}

#[test]
fn label() {
    use swarmladder_rust::basics::label;
    assert_eq!(label("  Café \t"), "Café");
    assert_eq!(label(" "), "");
}

#[test]
fn scalar_count() {
    use swarmladder_rust::basics::scalar_count;
    assert_eq!(scalar_count("é🦀"), 2);
    assert_eq!(scalar_count("e\u{301}"), 2);
}

#[test]
fn initial() {
    use swarmladder_rust::basics::initial;
    assert_eq!(initial(" 🦀"), Some('🦀'));
    assert_eq!(initial(" "), None);
}

#[test]
fn first_even() {
    use swarmladder_rust::basics::first_even;
    assert_eq!(first_even(&[3, -4, 2]), Some(-4));
    assert_eq!(first_even(&[1]), None);
}

#[test]
fn peak() {
    use swarmladder_rust::basics::peak;
    assert_eq!(peak(&[-8, -3]), Some(-3));
    assert_eq!(peak(&[]), None);
}

#[test]
fn position_of() {
    use swarmladder_rust::basics::position_of;
    assert_eq!(position_of(&[9, 4, 4], 4), Some(1));
    assert_eq!(position_of(&[], 1), None);
}

#[test]
fn positives() {
    use swarmladder_rust::basics::positives;
    assert_eq!(positives(&[2, 0, -1, 2]), vec![2, 2]);
    assert!(positives(&[]).is_empty());
}

#[test]
fn lengths() {
    use swarmladder_rust::basics::lengths;
    assert_eq!(lengths(&["a", "éé", ""]), vec![1, 2, 0]);
}

#[test]
fn pairs() {
    use swarmladder_rust::basics::pairs;
    assert_eq!(pairs(&[1, 4, 2]), vec![(1, 4), (4, 2)]);
    assert!(pairs(&[1]).is_empty());
}
