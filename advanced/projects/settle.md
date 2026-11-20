# Transactional ledger

Apply a sequence of transfers as one transaction. A failure must restore the complete opening ledger.

## Deliverable

Implement `settle` in [starter.rs](starter.rs).

Reject invalid account indices, self-transfers, insufficient funds and recipient overflow. Report the first zero-based transfer index. Successful transfers affect later ones. Commit only after every transfer succeeds.

## Acceptance example

```rust
use swarmladder_rust::advanced_projects::settle;
let mut a = [5, 0, 0];
assert_eq!(settle(&mut a, &[(0, 1, 5), (1, 2, 3)]), Ok(()));
assert_eq!(a, [0, 2, 3]);
assert_eq!(settle(&mut a, &[(2, 0, 2), (1, 2, 3)]), Err(1));
assert_eq!(a, [0, 2, 3]);
let mut b = [1, u32::MAX];
assert_eq!(settle(&mut b, &[(0, 1, 1)]), Err(0));
assert_eq!(b, [1, u32::MAX]);
assert_eq!(settle(&mut b, &[(2, 0, 0)]), Err(0));
assert_eq!(settle(&mut b, &[(0, 0, 0)]), Err(0));
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test advanced_projects settle -- --exact`.
3. Write a conservation test using u64 totals. Compare the snapshot with an undo log for a sparse batch.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
