# Parallel histogram

Combine scoped workers’ local histograms into a deterministic frequency table for byte input.

## Deliverable

Implement `histogram` in [starter.rs](starter.rs).

Reject zero workers. Divide nonempty input across at most the requested workers using scoped threads. Each worker owns a fixed-size array. Join and reduce without shared mutable counters.

## Acceptance example

```rust
use swarmladder_rust::advanced_projects::histogram;
let mut expected = [0; 256];
expected[0] = 1;
expected[255] = 2;
for n in [1, 2, 9] {
    assert_eq!(histogram(&[255, 0, 255], n), Some(expected));
}
assert_eq!(histogram(&[], 3), Some([0; 256]));
assert_eq!(histogram(&[1], 0), None);
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test advanced_projects histogram -- --exact`.
3. Compare to a sequential oracle on repeated bytes. Explain when creating threads costs more than counting.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
