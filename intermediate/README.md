# Intermediate Rust

Start after you can traverse slices and return an Option. Build validators, indexes and reusable algorithms with explicit error contracts.

## Workshops

1. [Validation with Result](workshop-1.md)
2. [Keyed collections](workshop-2.md)
3. [Generics and predicates](workshop-3.md)
4. [Iterator pipelines](workshop-4.md)

## Extended workshops

Each workshop has a worked example, a tracing question, five coding exercises, and two review questions with answers. Run the test command beside each exercise as you go.

5. [Ownership and mutation](practice/ownership_and_mutation.md)
6. [Domain errors](practice/domain_errors.md)
7. [Sets and maps](practice/sets_and_maps.md)
8. [Stacks and queues](practice/stacks_and_queues.md)
9. [Trait-based interfaces](practice/trait_based_interfaces.md)
10. [Ordering and binary search](practice/ordering_and_binary_search.md)
11. [Delimited data](practice/delimited_data.md)
12. [Closures and policies](practice/closures_and_policies.md)

## Projects

1. [Invoice roll-up](projects/invoices.md) — An accounting team receives invoice lines from several departments.
2. [Concordance](projects/index.md) — An editor needs a searchable index of the lines on which each word occurs.
3. [Room availability](projects/bookings.md) — A scheduler receives unsorted bookings.

## Level assessment

Complete the 52 functions and three projects, then run `cargo test --test intermediate --test intermediate_practice --test intermediate_projects`. Answer the knowledge checks in each workshop before opening its answer key. For each project, demonstrate its acceptance example and submit the final project-review exercise. Check your added cases against the reference with `cargo test --features reference --test intermediate --test intermediate_practice --test intermediate_projects`.
