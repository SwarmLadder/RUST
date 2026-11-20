#[test]
fn port() {
    use swarmladder_rust::intermediate::port;
    assert_eq!(port(" 443 "), Ok(443));
    assert_eq!(port("0"), Err("zero"));
    assert_eq!(port("65536"), Err("invalid"));
}

#[test]
fn required() {
    use swarmladder_rust::intermediate::required;
    assert_eq!(required(" x "), Ok("x"));
    assert_eq!(required(" "), Err("empty"));
}

#[test]
fn numbers() {
    use swarmladder_rust::intermediate::numbers;
    assert_eq!(numbers("1\n -3"), Ok(vec![1, -3]));
    assert_eq!(numbers("1\n\n5"), Err(2));
}

#[test]
fn word_counts() {
    use swarmladder_rust::intermediate::word_counts;
    let c = word_counts("Rust rust Rust");
    assert_eq!(c["Rust"], 2);
    assert_eq!(c.len(), 2);
}

#[test]
fn distinct() {
    use swarmladder_rust::intermediate::distinct;
    assert_eq!(distinct(&[3, -1, 3, 2]), vec![-1, 2, 3]);
    assert!(distinct(&[]).is_empty());
}

#[test]
fn first_unique() {
    use swarmladder_rust::intermediate::first_unique;
    assert_eq!(first_unique("a z a b"), Some("z"));
    assert_eq!(first_unique("a a"), None);
}

#[test]
fn larger() {
    use swarmladder_rust::intermediate::larger;
    assert_eq!(larger(&2, &9), &9);
    let a = String::from("x");
    let b = a.clone();
    assert!(std::ptr::eq(larger(&a, &b), &a));
}

#[test]
fn tally() {
    use swarmladder_rust::intermediate::tally;
    let mut visits = 0;
    assert_eq!(
        tally(&[1, 2, 4], |x| {
            visits += 1;
            x % 2 == 0
        }),
        2
    );
    assert_eq!(visits, 3);
}

#[test]
fn last_match() {
    use swarmladder_rust::intermediate::last_match;
    let mut seen = Vec::new();
    assert_eq!(
        last_match(&[4, 1, 2], |&x| {
            seen.push(x);
            x % 2 == 0
        }),
        Some(&2)
    );
    assert_eq!(seen, vec![4, 1, 2]);
}

#[test]
fn valid_ids() {
    use swarmladder_rust::intermediate::valid_ids;
    assert_eq!(valid_ids(&["3", "bad", " 3", "-1"]), vec![3, 3]);
}

#[test]
fn running() {
    use swarmladder_rust::intermediate::running;
    assert_eq!(
        running(&[i32::MAX, 1, -2]),
        vec![2147483647, 2147483648, 2147483646]
    );
    assert!(running(&[]).is_empty());
}

#[test]
fn dot() {
    use swarmladder_rust::intermediate::dot;
    assert_eq!(dot(&[2, -3], &[4, 5]), Some(-7));
    assert_eq!(dot(&[1], &[]), None);
    assert_eq!(dot(&[], &[]), Some(0));
}
