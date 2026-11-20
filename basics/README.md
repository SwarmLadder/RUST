# Basics Rust

Start here with stable Rust and Cargo. Practice with values, borrowed text and slices before building three small reports.

## Workshops

1. [Reading sensor values](workshop-1.md)
2. [Text and Unicode](workshop-2.md)
3. [Slices and searching](workshop-3.md)
4. [Building collections](workshop-4.md)

## Extended workshops

Each workshop has a worked example, a tracing question, five coding exercises, and two review questions with answers. Run the test command beside each exercise as you go.

5. [Integer arithmetic](practice/integer_arithmetic.md)
6. [Decisions and ranges](practice/decisions_and_ranges.md)
7. [Loops and termination](practice/loops_and_termination.md)
8. [Tuples and records](practice/tuples_and_records.md)
9. [Strings and formatting](practice/strings_and_formatting.md)
10. [Borrowing slices](practice/borrowing_slices.md)
11. [Enums and matching](practice/enums_and_matching.md)
12. [Small algorithms](practice/small_algorithms.md)

## Projects

1. [Trail elevation report](projects/trail.md) — A walking club records elevation at successive checkpoints.
2. [Pantry shopping list](projects/shopping.md) — A kitchen stocks named items.
3. [Scoreboard](projects/leaders.md) — Build a judge’s scorecard from named scores.

## Level assessment

Complete the 52 functions and three projects, then run `cargo test --test basics --test basics_practice --test basics_projects`. Answer the knowledge checks in each workshop before opening its answer key. For each project, demonstrate its acceptance example and submit the final project-review exercise. Check your added cases against the reference with `cargo test --features reference --test basics --test basics_practice --test basics_projects`.
