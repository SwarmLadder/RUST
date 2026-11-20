#[test]
fn invoices() {
    use swarmladder_rust::intermediate_projects::invoices;
    let m = invoices("QA,4\n UX , 7\nQA,3").unwrap();
    assert_eq!(
        m.into_iter().collect::<Vec<_>>(),
        vec![("QA".into(), 7), ("UX".into(), 7)]
    );
    assert_eq!(invoices("QA,18446744073709551615\nQA,1"), Err(2));
    assert_eq!(invoices(" ,4"), Err(1));
}

#[test]
fn index() {
    use swarmladder_rust::intermediate_projects::index;
    let m = index("Rust RUST\n\nRust rust!");
    assert_eq!(m["rust"], vec![1, 3]);
    assert_eq!(m["rust!"], vec![3]);
    assert!(index("").is_empty());
}

#[test]
fn bookings() {
    use swarmladder_rust::intermediate_projects::bookings;
    assert_eq!(
        bookings(&[(30, 40), (0, 20), (10, 12), (20, 30), (50, 60)]),
        Ok(vec![(0, 40), (50, 60)])
    );
    assert_eq!(bookings(&[(30, 40), (5, 5)]), Err(1));
    assert_eq!(bookings(&[(0, 1441)]), Err(0));
}
