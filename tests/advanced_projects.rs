#[test]
fn events() {
    use swarmladder_rust::advanced_projects::events;
    assert_eq!(
        events(&b"WARN hot\n INFO ok\nX INFO\nERROR\nWARN"[..]).unwrap(),
        [1, 2, 1]
    );
    assert!(events(&b"INFO\n\xff"[..]).is_err());
}

#[test]
fn histogram() {
    use swarmladder_rust::advanced_projects::histogram;
    let mut expected = [0; 256];
    expected[0] = 1;
    expected[255] = 2;
    for n in [1, 2, 9] {
        assert_eq!(histogram(&[255, 0, 255], n), Some(expected));
    }
    assert_eq!(histogram(&[], 3), Some([0; 256]));
    assert_eq!(histogram(&[1], 0), None);
}

#[test]
fn settle() {
    use swarmladder_rust::advanced_projects::settle;
    let mut a = [5, 0, 0];
    assert_eq!(settle(&mut a, &[(0, 1, 5), (1, 2, 3)]), Ok(()));
    assert_eq!(a, [0, 2, 3]);
    assert_eq!(settle(&mut a, &[(2, 0, 2), (1, 2, 3)]), Err(1));
    assert_eq!(a, [0, 2, 3]);
    let mut b = [1, u32::MAX];
    assert_eq!(settle(&mut b, &[(0, 1, 1)]), Err(0));
    assert_eq!(b, [1, u32::MAX]);
    assert_eq!(settle(&mut b, &[(2, 0, 0)]), Err(0));
    assert_eq!(settle(&mut b, &[(0, 0, 0)]), Err(0));
}
