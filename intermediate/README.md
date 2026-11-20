# Intermediate Rust

Start after you can traverse slices and return an Option. Build validators, indexes and reusable algorithms with explicit error contracts.

## Workshops

1. [Validation with Result](workshop-1.md)
2. [Keyed collections](workshop-2.md)
3. [Generics and predicates](workshop-3.md)
4. [Iterator pipelines](workshop-4.md)

## Projects

1. [Invoice roll-up](projects/invoices.md) — An accounting team receives invoice lines from several departments.
2. [Concordance](projects/index.md) — An editor needs a searchable index of the lines on which each word occurs.
3. [Room availability](projects/bookings.md) — A scheduler receives unsorted bookings.

## Level assessment

Complete the 12 functions and three projects, then run `cargo test --test intermediate --test intermediate_projects`. Answer the five knowledge checks in each workshop before opening its answer key. For each project, demonstrate its acceptance example and submit the final project-review exercise. Check your added cases against the reference with `cargo test --features reference --test intermediate --test intermediate_projects`.
