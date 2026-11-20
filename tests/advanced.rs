#[test]
fn before() {
    use swarmladder_rust::advanced::before;
    assert_eq!(before("a::b::c", "::"), Some("a"));
    assert_eq!(before("abc", ""), None);
    assert_eq!(before("abc", "z"), None);
}

#[test]
fn byte_slice() {
    use swarmladder_rust::advanced::byte_slice;
    assert_eq!(byte_slice("é!", 0, 2), Some("é"));
    assert_eq!(byte_slice("é!", 1, 2), None);
    assert_eq!(byte_slice("ab", 2, 1), None);
}

#[test]
fn field() {
    use swarmladder_rust::advanced::field;
    assert_eq!(field("bad; n = 2;n=3", "n"), Some("2"));
    assert_eq!(field("n=", "n"), Some(""));
    assert_eq!(field("x=1", "n"), None);
}

#[test]
fn line_count() {
    use swarmladder_rust::advanced::line_count;
    assert_eq!(line_count(&b"a\n\nb"[..]).unwrap(), 3);
    assert!(line_count(&[255u8][..]).is_err());
}

#[test]
fn write_rows() {
    use swarmladder_rust::advanced::write_rows;
    let mut buf = Vec::new();
    write_rows(&mut buf, &["a", ""]).unwrap();
    assert_eq!(buf, b"a\n\n");
    assert!(write_rows(&mut [0u8; 0][..], &["x"]).is_err());
}

#[test]
fn first_line() {
    use swarmladder_rust::advanced::first_line;
    assert_eq!(first_line(&b"a\nbb\n\xff"[..], "bb").unwrap(), Some(2));
    assert!(first_line(&[255u8][..], "x").is_err());
}

#[test]
fn chunk_size() {
    use swarmladder_rust::advanced::chunk_size;
    assert_eq!(chunk_size(10, 3), Some(4));
    assert_eq!(chunk_size(0, 3), Some(0));
    assert_eq!(chunk_size(usize::MAX, 1), Some(usize::MAX));
    assert_eq!(chunk_size(2, 0), None);
}

#[test]
fn parallel_sum() {
    use swarmladder_rust::advanced::parallel_sum;
    assert_eq!(parallel_sum(&[i32::MAX, 1, -2]), 2147483646);
    assert_eq!(parallel_sum(&[]), 0);
}

#[test]
fn drain() {
    use swarmladder_rust::advanced::drain;
    let (tx, rx) = std::sync::mpsc::channel();
    tx.send("b").unwrap();
    tx.send("a").unwrap();
    drop(tx);
    assert_eq!(drain(rx), vec!["b", "a"]);
}

#[test]
fn apply_delta() {
    use swarmladder_rust::advanced::apply_delta;
    assert_eq!(apply_delta(1, -2), None);
    assert_eq!(apply_delta(u32::MAX, 1), None);
    assert_eq!(apply_delta(1 << 31, i32::MIN), Some(0));
}

#[test]
fn batch() {
    use swarmladder_rust::advanced::batch;
    let mut b = 10;
    assert_eq!(batch(&mut b, &[5, -16]), Err(1));
    assert_eq!(b, 10);
    assert_eq!(batch(&mut b, &[-4, 1]), Ok(()));
    assert_eq!(b, 7);
}

#[test]
fn transfer() {
    use swarmladder_rust::advanced::transfer;
    let mut a = [3, u32::MAX];
    assert!(!transfer(&mut a, 0, 1, 1));
    assert_eq!(a, [3, u32::MAX]);
    assert!(transfer(&mut a, 1, 0, 2));
    assert_eq!(a, [5, u32::MAX - 2]);
}
