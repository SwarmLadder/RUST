# Streaming event counter

Process a log larger than available memory and tally severity without retaining event bodies.

## Deliverable

Implement `events` in [starter.rs](starter.rs).

Read lines incrementally. Count INFO, WARN and ERROR by first whitespace-delimited token, in that order. Ignore other lines. Propagate read and UTF-8 errors.

## Acceptance example

```rust
use swarmladder_rust::advanced_projects::events;
assert_eq!(
    events(&b"WARN hot\n INFO ok\nX INFO\nERROR\nWARN"[..]).unwrap(),
    [1, 2, 1]
);
assert!(events(&b"INFO\n\xff"[..]).is_err());
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test advanced_projects events -- --exact`.
3. Test a reader with a two-byte buffer. Describe memory usage in terms of longest line.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
