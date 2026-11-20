# Concordance

An editor needs a searchable index of the lines on which each word occurs.

## Deliverable

Implement `index` in [starter.rs](starter.rs).

Split on whitespace and apply ASCII lowercasing. Preserve punctuation. List each one-based line number at most once per word. Blank lines still count.

## Acceptance example

```rust
use swarmladder_rust::intermediate_projects::index;
let m = index("Rust RUST\n\nRust rust!");
assert_eq!(m["rust"], vec![1, 3]);
assert_eq!(m["rust!"], vec![3]);
assert!(index("").is_empty());
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test intermediate_projects index -- --exact`.
3. Compare a per-line set with checking the last recorded line. State the ordering assumption.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
