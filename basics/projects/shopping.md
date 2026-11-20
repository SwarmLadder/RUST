# Pantry shopping list

A kitchen stocks named items. Turn a shelf count into a shopping list that a volunteer can follow in aisle order.

## Deliverable

Implement `shopping` in [starter.rs](starter.rs).

Each row gives name, on-hand and target quantity. Keep only shortages, emit the missing amount and retain row order. Treat repeated labels as separate shelves.

## Acceptance example

```rust
use swarmladder_rust::basics_projects::shopping;
assert_eq!(
    shopping(&[("tea", 2, 5), ("jam", 4, 4), ("rice", 9, 2), ("tea", 0, 1)]),
    vec![("tea", 3), ("tea", 1)]
);
assert!(shopping(&[]).is_empty());
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test basics_projects shopping -- --exact`.
3. Explain how the list can borrow names when the shelf rows go out of scope.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
