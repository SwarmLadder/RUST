# Room availability

A scheduler receives unsorted bookings. Normalize them into occupied time spans for a room.

## Deliverable

Implement `bookings` in [starter.rs](starter.rs).

Use half-open minute ranges in a single day. Reject start >= end or end > 1440 with the first zero-based input index. Sort valid ranges, merging overlapping and touching spans.

## Acceptance example

```rust
use swarmladder_rust::intermediate_projects::bookings;
assert_eq!(
    bookings(&[(30, 40), (0, 20), (10, 12), (20, 30), (50, 60)]),
    Ok(vec![(0, 40), (50, 60)])
);
assert_eq!(bookings(&[(30, 40), (5, 5)]), Err(1));
assert_eq!(bookings(&[(0, 1441)]), Err(0));
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test intermediate_projects bookings -- --exact`.
3. Derive free time from the merged result and explain why validation precedes sorting.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
