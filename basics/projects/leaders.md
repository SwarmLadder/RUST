# Scoreboard

Build a judge’s scorecard from named scores. Several players may share first place.

## Deliverable

Implement `leaders` in [starter.rs](starter.rs).

Return the names attached to the maximum score, preserving input order and duplicate entries. Empty input gives an empty list. Negative scores are valid.

## Acceptance example

```rust
use swarmladder_rust::basics_projects::leaders;
assert_eq!(
    leaders(&[("Lin", -3), ("Ari", -1), ("Jo", -1)]),
    vec!["Ari", "Jo"]
);
assert!(leaders(&[]).is_empty());
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test basics_projects leaders -- --exact`.
3. Implement a single-pass alternative and compare the state it needs.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
