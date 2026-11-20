#[test]
fn token() {
    use swarmladder_rust::advanced_practice::zero_copy_parsing::token;
    assert_eq!(token("  é x"), Some(("é", " x")));
    assert_eq!(token(" "), None);
}

#[test]
fn quoted() {
    use swarmladder_rust::advanced_practice::zero_copy_parsing::quoted;
    assert_eq!(quoted("\"ab\"tail"), Some(("ab", "tail")));
    assert_eq!(quoted("\"bad"), None);
}

#[test]
fn header() {
    use swarmladder_rust::advanced_practice::zero_copy_parsing::header;
    assert_eq!(header("Kind: a:b"), Some(("Kind", "a:b")));
    assert_eq!(header("K1:x"), None);
}

#[test]
fn take_chars() {
    use swarmladder_rust::advanced_practice::zero_copy_parsing::take_chars;
    assert_eq!(take_chars("AéZ", 2), Some(("Aé", "Z")));
    assert_eq!(take_chars("", 0), Some(("", "")));
    assert_eq!(take_chars("a", 2), None);
}

#[test]
fn find_header() {
    use swarmladder_rust::advanced_practice::zero_copy_parsing::find_header;
    assert_eq!(find_header("bad\nX: one\nx: two", "x"), Some("one"));
    assert_eq!(find_header("Y:a", "z"), None);
}

#[test]
fn countdown() {
    use swarmladder_rust::advanced_practice::lazy_iterator_construction::countdown;
    assert_eq!(countdown(2).collect::<Vec<_>>(), [2, 1, 0]);
    assert_eq!(countdown(u32::MAX).take(1).next(), Some(u32::MAX));
}

#[test]
fn stride() {
    use swarmladder_rust::advanced_practice::lazy_iterator_construction::stride;
    assert_eq!(stride(&[1, 2, 3], 2).collect::<Vec<_>>(), [1, 3]);
    assert_eq!(stride(&[1], 0).next(), None);
}

#[test]
fn prefixes() {
    use swarmladder_rust::advanced_practice::lazy_iterator_construction::prefixes;
    assert_eq!(
        prefixes(&[u32::MAX, 1]).collect::<Vec<_>>(),
        [4294967295, 4294967296]
    );
    assert_eq!(prefixes(&[]).next(), None);
}

#[test]
fn fib() {
    use swarmladder_rust::advanced_practice::lazy_iterator_construction::fib;
    assert_eq!(fib().take(6).collect::<Vec<_>>(), [0, 1, 1, 2, 3, 5]);
    assert_eq!(fib().last(), Some(2971215073));
}

#[test]
fn runs() {
    use swarmladder_rust::advanced_practice::lazy_iterator_construction::runs;
    assert_eq!(
        runs(&[1, 1, 2, 1]).collect::<Vec<_>>(),
        [(1, 2), (2, 1), (1, 1)]
    );
    assert_eq!(runs(&[]).next(), None);
}

#[test]
fn nodes() {
    use swarmladder_rust::advanced_practice::recursive_data_with_box::nodes;
    use swarmladder_rust::advanced_practice::Expr;
    assert_eq!(nodes(&Expr::lit(4)), 1);
    assert_eq!(nodes(&Expr::sum(Expr::lit(2), Expr::lit(3))), 3);
}

#[test]
fn depth() {
    use swarmladder_rust::advanced_practice::recursive_data_with_box::depth;
    use swarmladder_rust::advanced_practice::Expr;
    assert_eq!(depth(&Expr::lit(0)), 1);
    assert_eq!(
        depth(&Expr::sum(
            Expr::lit(1),
            Expr::product(Expr::lit(2), Expr::lit(3))
        )),
        3
    );
}

#[test]
fn compute() {
    use swarmladder_rust::advanced_practice::recursive_data_with_box::compute;
    use swarmladder_rust::advanced_practice::Expr;
    assert_eq!(
        compute(&Expr::product(Expr::lit(-2), Expr::lit(4))),
        Some(-8)
    );
    assert_eq!(compute(&Expr::sum(Expr::lit(i32::MAX), Expr::lit(1))), None);
}

#[test]
fn postfix() {
    use swarmladder_rust::advanced_practice::recursive_data_with_box::postfix;
    use swarmladder_rust::advanced_practice::Expr;
    assert_eq!(
        postfix(&Expr::sum(
            Expr::lit(1),
            Expr::product(Expr::lit(2), Expr::lit(3))
        )),
        "1 2 3 * +"
    );
    assert_eq!(postfix(&Expr::lit(-1)), "-1");
}

#[test]
fn map_literals() {
    use swarmladder_rust::advanced_practice::recursive_data_with_box::map_literals;
    use swarmladder_rust::advanced_practice::Expr;
    assert_eq!(
        map_literals(Expr::sum(Expr::lit(1), Expr::lit(2)), &|v| v + 1),
        Expr::sum(Expr::lit(2), Expr::lit(3))
    );
    assert_eq!(map_literals(Expr::lit(-2), &i32::abs), Expr::lit(2));
}

#[test]
fn valid_graph() {
    use swarmladder_rust::advanced_practice::graph_traversal::valid_graph;
    assert_eq!(valid_graph(&[vec![1], vec![]]), true);
    assert_eq!(valid_graph(&[vec![1]]), false);
}

#[test]
fn transpose() {
    use swarmladder_rust::advanced_practice::graph_traversal::transpose;
    assert_eq!(
        transpose(&[vec![1, 1], vec![]]),
        Some(vec![vec![], vec![0, 0]])
    );
    assert_eq!(transpose(&[vec![2]]), None);
}

#[test]
fn distances() {
    use swarmladder_rust::advanced_practice::graph_traversal::distances;
    assert_eq!(
        distances(&[vec![1], vec![0], vec![]], 0),
        Some(vec![Some(0), Some(1), None])
    );
    assert_eq!(distances(&[], 0), None);
}

#[test]
fn path() {
    use swarmladder_rust::advanced_practice::graph_traversal::path;
    assert_eq!(
        path(&[vec![1, 2], vec![3], vec![3], vec![]], 0, 3),
        Some(vec![0, 1, 3])
    );
    assert_eq!(path(&[vec![], vec![]], 0, 1), None);
}

#[test]
fn topological() {
    use swarmladder_rust::advanced_practice::graph_traversal::topological;
    assert_eq!(
        topological(&[vec![2], vec![2], vec![]]),
        Some(vec![0, 1, 2])
    );
    assert_eq!(topological(&[vec![0]]), None);
}

#[test]
fn word() {
    use swarmladder_rust::advanced_practice::binary_framing::word;
    assert_eq!(word(&[1, 2, 7]), Some(258));
    assert_eq!(word(&[1]), None);
}

#[test]
fn frame() {
    use swarmladder_rust::advanced_practice::binary_framing::frame;
    assert_eq!(frame(&[0, 1, 9, 7]), Some((&[9][..], &[7][..])));
    assert_eq!(frame(&[0, 2, 1]), None);
}

#[test]
fn encode() {
    use swarmladder_rust::advanced_practice::binary_framing::encode;
    assert_eq!(encode(&[7, 8]), Some(vec![0, 2, 7, 8]));
    assert_eq!(encode(&vec![0; 65536]), None);
}

#[test]
fn frames() {
    use swarmladder_rust::advanced_practice::binary_framing::frames;
    assert_eq!(frames(&[0, 0, 0, 1, 4]), Ok(vec![&[][..], &[4][..]]));
    assert_eq!(frames(&[0, 0, 1]), Err(2));
}

#[test]
fn checksum() {
    use swarmladder_rust::advanced_practice::binary_framing::checksum;
    assert_eq!(checksum(&[255, 2]), 255);
    assert_eq!(checksum(&[]), 0);
}

#[test]
fn share() {
    use swarmladder_rust::advanced_practice::sharing_and_synchronization::share;
    let (a, b) = share(String::from("x"));
    assert!(std::sync::Arc::ptr_eq(&a, &b));
    assert_eq!(&*a, "x");
}

#[test]
fn ticket() {
    use swarmladder_rust::advanced_practice::sharing_and_synchronization::ticket;
    let c = std::sync::atomic::AtomicUsize::new(usize::MAX);
    assert_eq!(ticket(&c), usize::MAX);
    assert_eq!(ticket(&c), 0);
}

#[test]
fn try_append() {
    use swarmladder_rust::advanced_practice::sharing_and_synchronization::try_append;
    let m = std::sync::Mutex::new(vec![1]);
    let lock = m.lock().unwrap();
    assert!(!try_append(&m, 2));
    drop(lock);
    assert!(try_append(&m, 3));
    assert_eq!(*m.lock().unwrap(), [1, 3]);
}

#[test]
fn parallel_max() {
    use swarmladder_rust::advanced_practice::sharing_and_synchronization::parallel_max;
    assert_eq!(parallel_max(&[-8, -1, -4]), Some(-1));
    assert_eq!(parallel_max(&[]), None);
}

#[test]
fn ordered_jobs() {
    use swarmladder_rust::advanced_practice::sharing_and_synchronization::ordered_jobs;
    let jobs: Vec<_> = (0..4).map(|v| move || v * v).collect();
    assert_eq!(ordered_jobs(jobs).unwrap(), [0, 1, 4, 9]);
    let jobs: Vec<_> = (0..2).map(|n| move || assert!(n > 5)).collect();
    assert!(ordered_jobs(jobs).is_err());
}

#[test]
fn read_number() {
    use swarmladder_rust::advanced_practice::working_with_generic_streams::read_number;
    assert_eq!(read_number(&[1, 2, 0, 0][..]).unwrap(), 513);
    assert_eq!(
        read_number(&[9][..]).unwrap_err().kind(),
        std::io::ErrorKind::UnexpectedEof
    );
}

#[test]
fn bounded() {
    use swarmladder_rust::advanced_practice::working_with_generic_streams::bounded;
    let mut r = &b"abcd"[..];
    let mut w = vec![];
    assert_eq!(bounded(&mut r, &mut w, 2).unwrap(), 2);
    assert_eq!(w, b"ab");
    assert_eq!(r, b"cd");
}

#[test]
fn write_record() {
    use swarmladder_rust::advanced_practice::working_with_generic_streams::write_record;
    let mut v = vec![];
    write_record(&mut v, b"x").unwrap();
    assert_eq!(v, [0, 1, 120]);
    assert!(write_record(&mut v, &vec![0; 65536]).is_err());
    assert_eq!(v, [0, 1, 120]);
}

#[test]
fn last_nonblank() {
    use swarmladder_rust::advanced_practice::working_with_generic_streams::last_nonblank;
    assert_eq!(
        last_nonblank(&b"a\n b \r\n \n"[..]).unwrap(),
        Some(" b ".into())
    );
    assert!(last_nonblank(&b"\xff"[..]).is_err());
}

#[test]
fn peek() {
    use swarmladder_rust::advanced_practice::working_with_generic_streams::peek;
    let mut r = &b"az"[..];
    assert_eq!(peek(&mut r).unwrap(), Some(97));
    assert_eq!(r, b"az");
    assert_eq!(peek(&mut &b""[..]).unwrap(), None);
}

#[test]
fn apply() {
    use swarmladder_rust::advanced_practice::versioned_state_machines::apply;
    use swarmladder_rust::advanced_practice::Revision;
    assert_eq!(
        apply(
            Revision {
                version: 2,
                value: 4
            },
            2,
            8
        ),
        Some(Revision {
            version: 3,
            value: 8
        })
    );
    assert_eq!(
        apply(
            Revision {
                version: u32::MAX,
                value: 0
            },
            u32::MAX,
            2
        ),
        None
    );
}

#[test]
fn replay() {
    use swarmladder_rust::advanced_practice::versioned_state_machines::replay;
    use swarmladder_rust::advanced_practice::Revision;
    assert_eq!(
        replay(
            Revision {
                version: 0,
                value: 0
            },
            &[(0, 2), (1, 5)]
        ),
        Ok(Revision {
            version: 2,
            value: 5
        })
    );
    assert_eq!(
        replay(
            Revision {
                version: 0,
                value: 0
            },
            &[(0, 1), (0, 2)]
        ),
        Err(1)
    );
}

#[test]
fn commit() {
    use swarmladder_rust::advanced_practice::versioned_state_machines::commit;
    use swarmladder_rust::advanced_practice::Revision;
    let mut s = Revision {
        version: 0,
        value: 4,
    };
    assert_eq!(commit(&mut s, &[(0, 8), (0, 2)]), Err(1));
    assert_eq!(
        s,
        Revision {
            version: 0,
            value: 4
        }
    );
    assert_eq!(commit(&mut s, &[(0, 8)]), Ok(()));
    assert_eq!(s.value, 8);
}

#[test]
fn history() {
    use swarmladder_rust::advanced_practice::versioned_state_machines::history;
    use swarmladder_rust::advanced_practice::Revision;
    let s = Revision {
        version: 4,
        value: 1,
    };
    assert_eq!(history(s, &[]), Ok(vec![s]));
    assert_eq!(
        history(s, &[(4, 9)]).unwrap(),
        [
            s,
            Revision {
                version: 5,
                value: 9
            }
        ]
    );
    assert_eq!(history(s, &[(3, 9)]), Err(0));
}

#[test]
fn rollback() {
    use swarmladder_rust::advanced_practice::versioned_state_machines::rollback;
    use swarmladder_rust::advanced_practice::Revision;
    let a = Revision {
        version: 7,
        value: -1,
    };
    let b = Revision {
        version: 8,
        value: 3,
    };
    assert_eq!(rollback(&[a, b], 7), Some(a));
    assert_eq!(rollback(&[a, b], 6), None);
}
