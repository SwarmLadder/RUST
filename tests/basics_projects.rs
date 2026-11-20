#[test]
fn trail() {
    use swarmladder_rust::basics_projects::trail;
    assert_eq!(trail(&[]), None);
    assert_eq!(trail(&[3, 7, 2, 7]), Some((9, 1)));
    assert_eq!(trail(&[i32::MIN, i32::MAX]), Some((u32::MAX as u64, 1)));
}

#[test]
fn shopping() {
    use swarmladder_rust::basics_projects::shopping;
    assert_eq!(
        shopping(&[("tea", 2, 5), ("jam", 4, 4), ("rice", 9, 2), ("tea", 0, 1)]),
        vec![("tea", 3), ("tea", 1)]
    );
    assert!(shopping(&[]).is_empty());
}

#[test]
fn leaders() {
    use swarmladder_rust::basics_projects::leaders;
    assert_eq!(
        leaders(&[("Lin", -3), ("Ari", -1), ("Jo", -1)]),
        vec!["Ari", "Jo"]
    );
    assert!(leaders(&[]).is_empty());
}
