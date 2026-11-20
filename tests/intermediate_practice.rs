#[test]
fn take_title() {
    use swarmladder_rust::intermediate_practice::ownership_and_mutation::take_title;
    let mut s = String::from("old");
    assert_eq!(take_title(&mut s), "old");
    assert!(s.is_empty());
}

#[test]
fn rename() {
    use swarmladder_rust::intermediate_practice::ownership_and_mutation::rename;
    let mut s = String::from("a");
    assert_eq!(rename(&mut s, "b".into()), "a");
    assert_eq!(s, "b");
}

#[test]
fn pop_pair() {
    use swarmladder_rust::intermediate_practice::ownership_and_mutation::pop_pair;
    let mut v = vec!["a".into(), "b".into()];
    assert_eq!(pop_pair(&mut v), Some(("a".into(), "b".into())));
    v.push("c".into());
    assert_eq!(pop_pair(&mut v), None);
    assert_eq!(v, ["c"]);
}

#[test]
fn partition_titles() {
    use swarmladder_rust::intermediate_practice::ownership_and_mutation::partition_titles;
    assert_eq!(
        partition_titles(vec![" ".into(), " A ".into()]),
        (vec![String::from(" A ")], vec![String::from(" ")])
    );
    assert_eq!(partition_titles(vec![]), (vec![], vec![]));
}

#[test]
fn join_owned() {
    use swarmladder_rust::intermediate_practice::ownership_and_mutation::join_owned;
    assert_eq!(join_owned("swarm".into(), " lab"), "swarm lab");
    assert_eq!(join_owned("".into(), "é"), "é");
}

#[test]
fn key() {
    use swarmladder_rust::intermediate_practice::domain_errors::key;
    use swarmladder_rust::intermediate_practice::ConfigError;
    assert_eq!(key(" a "), Ok("a"));
    assert_eq!(key(" "), Err(ConfigError::EmptyKey));
}

#[test]
fn quantity() {
    use swarmladder_rust::intermediate_practice::domain_errors::quantity;
    use swarmladder_rust::intermediate_practice::ConfigError;
    assert_eq!(quantity(" 0"), Ok(0));
    assert_eq!(quantity("-1"), Err(ConfigError::InvalidValue));
}

#[test]
fn setting() {
    use swarmladder_rust::intermediate_practice::domain_errors::setting;
    use swarmladder_rust::intermediate_practice::ConfigError;
    assert_eq!(setting("x = 9"), Ok(("x", 9)));
    assert_eq!(setting(" "), Err(ConfigError::MissingEquals));
    assert_eq!(setting(" =bad"), Err(ConfigError::EmptyKey));
}

#[test]
fn settings() {
    use swarmladder_rust::intermediate_practice::domain_errors::settings;
    use swarmladder_rust::intermediate_practice::ConfigError;
    assert_eq!(settings("a=1\nb=2"), Ok(vec![("a", 1), ("b", 2)]));
    assert_eq!(settings("a=1\nb=?"), Err((2, ConfigError::InvalidValue)));
}

#[test]
fn all_errors() {
    use swarmladder_rust::intermediate_practice::domain_errors::all_errors;
    use swarmladder_rust::intermediate_practice::ConfigError;
    assert_eq!(
        all_errors("x\na=2\n="),
        vec![(1, ConfigError::MissingEquals), (3, ConfigError::EmptyKey)]
    );
    assert_eq!(all_errors(""), vec![]);
}

#[test]
fn missing() {
    use swarmladder_rust::intermediate_practice::sets_and_maps::missing;
    assert_eq!(missing(&[3, 1, 1], &[2, 3]), vec![1]);
    assert_eq!(missing(&[], &[]), vec![]);
}

#[test]
fn common() {
    use swarmladder_rust::intermediate_practice::sets_and_maps::common;
    assert_eq!(common(&[4, 2, 4], &[4, 2]), vec![2, 4]);
    assert_eq!(common(&[1], &[]), vec![]);
}

#[test]
fn invert() {
    use swarmladder_rust::intermediate_practice::sets_and_maps::invert;
    let m = invert(&[("b", 1), ("a", 1), ("c", 2)]);
    assert_eq!(m[&1], ["b", "a"]);
    assert_eq!(m[&2], ["c"]);
}

#[test]
fn duplicate() {
    use swarmladder_rust::intermediate_practice::sets_and_maps::duplicate;
    assert_eq!(duplicate(&[2, 1, 1, 2]), Some(1));
    assert_eq!(duplicate(&[3, 1]), None);
}

#[test]
fn vote() {
    use swarmladder_rust::intermediate_practice::sets_and_maps::vote;
    assert_eq!(vote(&["z", "a", "z", "a"]), Some("a".into()));
    assert_eq!(vote(&[]), None);
}

#[test]
fn balanced() {
    use swarmladder_rust::intermediate_practice::stacks_and_queues::balanced;
    assert_eq!(balanced("a{[()]}"), true);
    assert_eq!(balanced("([)]"), false);
    assert_eq!(balanced("("), false);
}

#[test]
fn undo() {
    use swarmladder_rust::intermediate_practice::stacks_and_queues::undo;
    assert_eq!(undo(&[None, Some(1), Some(2), None]), vec![1]);
    assert_eq!(undo(&[]), vec![]);
}

#[test]
fn round_robin() {
    use swarmladder_rust::intermediate_practice::stacks_and_queues::round_robin;
    assert_eq!(
        round_robin(&[("A", 2), ("Z", 0), ("B", 1)]),
        vec!["A", "B", "A"]
    );
    assert_eq!(round_robin(&[]), Vec::<String>::new());
}

#[test]
fn recent() {
    use swarmladder_rust::intermediate_practice::stacks_and_queues::recent;
    assert_eq!(recent(&[1, 2, 3], 2), vec![2, 3]);
    assert_eq!(recent(&[1], 0), vec![]);
}

#[test]
fn evaluate() {
    use swarmladder_rust::intermediate_practice::stacks_and_queues::evaluate;
    assert_eq!(evaluate(&["9", "2", "-"]), Some(7));
    assert_eq!(evaluate(&["1", "2"]), None);
    assert_eq!(evaluate(&["2147483647", "1", "+"]), None);
}

#[test]
fn render() {
    use swarmladder_rust::intermediate_practice::trait_based_interfaces::render;
    assert_eq!(render(&7u32), "[7]");
    assert_eq!(render(&String::from("hi")), "[hi]");
}

#[test]
fn labels() {
    use swarmladder_rust::intermediate_practice::trait_based_interfaces::labels;
    assert_eq!(labels(&[&7u32, &String::from("hi")]), vec!["7", "hi"]);
    assert_eq!(labels(&[]), Vec::<String>::new());
}

#[test]
fn render_all() {
    use swarmladder_rust::intermediate_practice::trait_based_interfaces::render_all;
    assert_eq!(render_all(&[2u32, 5]), "[2],[5]");
    assert_eq!(render_all(&[] as &[u32]), "");
}

#[test]
fn same_label() {
    use swarmladder_rust::intermediate_practice::trait_based_interfaces::same_label;
    assert_eq!(same_label(&3u32, &String::from("3")), true);
    assert_eq!(same_label(&2u32, &3u32), false);
}

#[test]
fn longest_label() {
    use swarmladder_rust::intermediate_practice::trait_based_interfaces::longest_label;
    assert_eq!(
        longest_label(&[&String::from("é"), &9u32]),
        Some("é".into())
    );
    assert_eq!(longest_label(&[]), None);
}

#[test]
fn lower() {
    use swarmladder_rust::intermediate_practice::ordering_and_binary_search::lower;
    assert_eq!(lower(&[1, 3, 3, 8], 3), 1);
    assert_eq!(lower(&[1], 2), 1);
}

#[test]
fn equal_range() {
    use swarmladder_rust::intermediate_practice::ordering_and_binary_search::equal_range;
    assert_eq!(equal_range(&[1, 3, 3, 8], 3), 1..3);
    assert_eq!(equal_range(&[1, 5], 2), 1..1);
}

#[test]
fn rank() {
    use swarmladder_rust::intermediate_practice::ordering_and_binary_search::rank;
    assert_eq!(
        rank(&[("b".into(), 5), ("a".into(), 5), ("z".into(), 8)]),
        vec!["z", "a", "b"]
    );
    assert_eq!(rank(&[]), Vec::<String>::new());
}

#[test]
fn median() {
    use swarmladder_rust::intermediate_practice::ordering_and_binary_search::median;
    assert_eq!(median(&[8, 1, 3, 2]), Some(2));
    assert_eq!(median(&[]), None);
}

#[test]
fn insert_unique() {
    use swarmladder_rust::intermediate_practice::ordering_and_binary_search::insert_unique;
    let mut v = vec![1, 4];
    assert!(insert_unique(&mut v, 2));
    assert!(!insert_unique(&mut v, 4));
    assert_eq!(v, [1, 2, 4]);
}

#[test]
fn cells() {
    use swarmladder_rust::intermediate_practice::delimited_data::cells;
    assert_eq!(cells("a\t\t"), vec!["a", "", ""]);
    assert_eq!(cells(""), vec![""]);
}

#[test]
fn escape() {
    use swarmladder_rust::intermediate_practice::delimited_data::escape;
    assert_eq!(escape("a|b"), "a\\|b");
    assert_eq!(escape("\\"), "\\\\");
}

#[test]
fn decode_row() {
    use swarmladder_rust::intermediate_practice::delimited_data::decode_row;
    assert_eq!(decode_row("a\\|b|"), Some(vec!["a|b".into(), "".into()]));
    assert_eq!(decode_row("a\\"), None);
    assert_eq!(decode_row("\\q"), None);
}

#[test]
fn encode_row() {
    use swarmladder_rust::intermediate_practice::delimited_data::encode_row;
    assert_eq!(encode_row(&["|", ""]), Some("\\||".into()));
    assert_eq!(encode_row(&[]), None);
}

#[test]
fn widths() {
    use swarmladder_rust::intermediate_practice::delimited_data::widths;
    assert_eq!(widths("a\tb\nc\td"), Ok(2));
    assert_eq!(widths("a\tb\nc"), Err(2));
    assert_eq!(widths(""), Ok(0));
}

#[test]
fn first_bad() {
    use swarmladder_rust::intermediate_practice::closures_and_policies::first_bad;
    let mut calls = 0;
    assert_eq!(
        first_bad(&[2, 1, 4], |x| {
            calls += 1;
            *x % 2 == 0
        }),
        Some(1)
    );
    assert_eq!(calls, 2);
}

#[test]
fn transform() {
    use swarmladder_rust::intermediate_practice::closures_and_policies::transform;
    assert_eq!(transform(vec![2, 3], |n| n * 2), vec![4, 6]);
    assert_eq!(transform(vec![String::from("hi")], |s| s.len()), vec![2]);
}

#[test]
fn retry() {
    use swarmladder_rust::intermediate_practice::closures_and_policies::retry;
    let mut k = 0;
    assert_eq!(
        retry(5, || {
            k += 1;
            if k == 2 {
                Ok(k)
            } else {
                Err(0)
            }
        }),
        Some(Ok(2))
    );
    assert_eq!(k, 2);
    assert_eq!(retry(0, || Err::<(), _>(4)), None);
    assert_eq!(retry(2, || Err::<(), _>(4)), Some(Err(4)));
}

#[test]
fn adjacent() {
    use swarmladder_rust::intermediate_practice::closures_and_policies::adjacent;
    assert_eq!(adjacent(&[1, 2, 3], |a, b| a < b), true);
    assert_eq!(adjacent(&[2, 1], |a, b| a < b), false);
}

#[test]
fn fold_until() {
    use swarmladder_rust::intermediate_practice::closures_and_policies::fold_until;
    assert_eq!(
        fold_until(&[2, 4, -1, 8], 0, |a, b| (b >= 0).then(|| a + b)),
        6
    );
    assert_eq!(fold_until(&[], 7, i32::checked_add), 7);
}
