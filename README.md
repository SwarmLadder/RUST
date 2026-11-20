# Rust foundations and practical Rust

Build a study-log analyzer in eight exercises. Start with a single session and finish with a command that groups a whole week of sessions by topic. Both modules use stable Rust, Cargo and the standard library.

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

## Start

Install stable Rust and Cargo using [rustup](https://rustup.rs/), then run:

```sh
git clone https://github.com/SwarmLadder/RUST
cd RUST
cargo check --all-targets
cargo test lesson1
```

Open [the exercise file](src/exercises.rs). Each starter compiles and stops at an intentional `todo!()` until you implement it. Run the lesson's tests after each edit. After module 1, run `cargo test --test course` and `cargo run --example study_log`. After module 2, run `cargo test` for the full suite.

## Check a worked solution

`cargo test --features reference` runs the same tests against the worked solutions. `cargo run --example study_log --features reference` prints the completed report. Use the default commands when checking your own work.

The course runs locally with Cargo and has no external crate dependencies. SwarmLadder Lab browser execution for Rust is a separate milestone.
