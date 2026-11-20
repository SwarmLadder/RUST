# Invoice roll-up

An accounting team receives invoice lines from several departments. Produce deterministic totals or identify the first unusable line.

## Deliverable

Implement `invoices` in [starter.rs](starter.rs).

Read one department,amount per line. Trim both fields, reject blank departments, parse unsigned integer amounts, reject sum overflow. Errors report the one-based source line. Empty input is valid.

## Acceptance example

```rust
use swarmladder_rust::intermediate_projects::invoices;
let m = invoices("QA,4\n UX , 7\nQA,3").unwrap();
assert_eq!(
    m.into_iter().collect::<Vec<_>>(),
    vec![("QA".into(), 7), ("UX".into(), 7)]
);
assert_eq!(invoices("QA,18446744073709551615\nQA,1"), Err(2));
assert_eq!(invoices(" ,4"), Err(1));
```

## Project review

1. Trace the example by hand. Record one representative input and one boundary case.
2. Implement the function and check it with `cargo test --test intermediate_projects invoices -- --exact`.
3. Add a malformed amount after two valid lines. Explain why callers never see a partial report.

Compare your implementation with [the worked projects](reference.rs) after your tests pass.
