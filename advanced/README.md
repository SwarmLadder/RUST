# Advanced Rust

Start after you can use Result, generic functions and iterator adapters. Practice lifetime design, streaming, concurrency and transactional updates.

## Workshops

1. [Lifetimes and borrowed views](workshop-1.md)
2. [Streaming I/O](workshop-2.md)
3. [Concurrency and message passing](workshop-3.md)
4. [Atomic state transitions](workshop-4.md)

## Projects

1. [Streaming event counter](projects/events.md) — Process a log larger than available memory and tally severity without retaining event bodies.
2. [Parallel histogram](projects/histogram.md) — Combine scoped workers’ local histograms into a deterministic frequency table for byte input.
3. [Transactional ledger](projects/settle.md) — Apply a sequence of transfers as one transaction.

## Level assessment

Complete the 12 functions and three projects, then run `cargo test --test advanced --test advanced_projects`. Answer the five knowledge checks in each workshop before opening its answer key. For each project, demonstrate its acceptance example and submit the final project-review exercise. Check your added cases against the reference with `cargo test --features reference --test advanced --test advanced_projects`.
