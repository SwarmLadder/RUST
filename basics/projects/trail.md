# Trail elevation report

A walking club records elevation at successive checkpoints. Compute total uphill travel and identify the first highest checkpoint. Sketch the output for a flat route before writing the loop.

## Deliverable

Implement `trail` in [starter.rs](starter.rs).

An empty route has no report. The first sample contributes no ascent. Count only positive changes, widening before subtracting. Return ascent and the zero-based index of the first summit.

## Acceptance example

```rust
use swarmladder_rust::basics_projects::trail;
assert_eq!(trail(&[]), None);
assert_eq!(trail(&[3, 7, 2, 7]), Some((9, 1)));
assert_eq!(trail(&[i32::MIN, i32::MAX]), Some((u32::MAX as u64, 1)));
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test basics_projects trail -- --exact`.
3. Add a test for a single checkpoint and explain the tie rule.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
