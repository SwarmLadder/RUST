# SwarmLadder Rust

Learn Rust locally with Cargo. The course has three levels. Each level includes 12 workshops, 52 coding exercises, 36 review questions with explained answers, and three projects with acceptance tests and worked solutions.

| Level | What you will practice | What you will build |
| --- | --- | --- |
| [Basics](basics/README.md) | Values, conditions, UTF-8 text, slices and vectors | Trail elevation report, pantry shopping list, scoreboard |
| [Intermediate](intermediate/README.md) | Ownership, domain errors, collections, traits, ordering, data formats and closures | Invoice roll-up, concordance, room scheduler |
| [Advanced](advanced/README.md) | Borrowed views, streaming I/O, scoped threads and rollback | Event counter, parallel histogram, transactional ledger |

## Your first exercise

Install stable Rust using [rustup](https://rustup.rs/). Then run:

```sh
git clone https://github.com/SwarmLadder/RUST
cd RUST
cargo check --all-targets
cargo test --test basics temperature -- --exact
```

The starter test fails at an intentional `todo!()`. Read [Reading sensor values](basics/workshop-1.md), edit `temperature` in [basics/exercises.rs](basics/exercises.rs), and rerun the same command. Use the default Cargo features to test your own code. Use `cargo test --features reference` to check the complete answer key, including examples from the guides.

## Guided study-log project

These eight linked lessons build a command-line application across two modules. Try module 1 alongside Basics and module 2 alongside Intermediate.

## Module 1 — foundations

| Lesson | Implement | Check |
| --- | --- | --- |
| [Divide a session into blocks](lessons/01-blocks.md) | `blocks` | `cargo test lesson1` |
| [Borrow a title](lessons/02-titles.md) | `title` | `cargo test lesson2` |
| [Parse a session](lessons/03-entries.md) | `parse_entry` | `cargo test lesson3` |
| [Project — summarize a study log](lessons/04-summary.md) | `summarize` | `cargo test lesson4` |

## Module 2 — build a command-line report

Start after completing the parser in lesson 3. Each lesson includes a worked example, an implementation contract, debugging guidance and a focused test command.

| Lesson | Implement | Check |
| --- | --- | --- |
| [Keep owned records](lessons/05-owned-records.md) | `sessions` | `cargo test lesson5` |
| [Group minutes by topic](lessons/06-topic-totals.md) | `totals` | `cargo test lesson6` |
| [Render predictable output](lessons/07-render.md) | `render` | `cargo test lesson7` |
| [Project — command-line report](lessons/08-cli.md) | `report` | `cargo test lesson8` |

After module 1, run `cargo test --test course`. After module 2, run `cargo test --test report`. Run the completed example with `cargo run --example study_log --features reference`.
