# Advanced Rust

Start after you can use Result, generic functions and iterator adapters. Practice lifetime design, streaming, concurrency and transactional updates.

## Workshops

1. [Lifetimes and borrowed views](workshop-1.md)
2. [Streaming I/O](workshop-2.md)
3. [Concurrency and message passing](workshop-3.md)
4. [Atomic state transitions](workshop-4.md)

## Extended workshops

Each workshop has a worked example, a tracing question, five coding exercises, and two review questions with answers. Run the test command beside each exercise as you go.

5. [Zero-copy parsing](practice/zero_copy_parsing.md)
6. [Lazy iterator construction](practice/lazy_iterator_construction.md)
7. [Recursive data with Box](practice/recursive_data_with_box.md)
8. [Graph traversal](practice/graph_traversal.md)
9. [Binary framing](practice/binary_framing.md)
10. [Sharing and synchronization](practice/sharing_and_synchronization.md)
11. [Working with generic streams](practice/working_with_generic_streams.md)
12. [Versioned state machines](practice/versioned_state_machines.md)

## Projects

1. [Streaming event counter](projects/events.md) — Process a log larger than available memory and tally severity without retaining event bodies.
2. [Parallel histogram](projects/histogram.md) — Combine scoped workers’ local histograms into a deterministic frequency table for byte input.
3. [Transactional ledger](projects/settle.md) — Apply a sequence of transfers as one transaction.

## Level assessment

Complete the 52 functions and three projects, then run `cargo test --test advanced --test advanced_practice --test advanced_projects`. Answer the knowledge checks in each workshop before opening its answer key. For each project, demonstrate its acceptance example and submit the final project-review exercise. Check your added cases against the reference with `cargo test --features reference --test advanced --test advanced_practice --test advanced_projects`.

## Engineering mastery extension — A01–A04

After the **52 existing Advanced functions and three Advanced projects**, complete the [four additional Advanced mastery challenges](../mastery/advanced.md): bounded binary frame decoding (A01), scoped-thread byte histograms (A02), all-or-nothing balance updates (A03), and race-safe atomic capacity reservations (A04).

Implement them in [mastery/starter.rs](../mastery/starter.rs), starting with `cargo test --test mastery a01_frames`. The `reference` feature is for testing the course key, not grading unfinished exercises. Create an [offline reliability portfolio](../PORTFOLIO.md), then use the [final assessment](../ASSESSMENT.md) to check your evidence and safety decisions.
