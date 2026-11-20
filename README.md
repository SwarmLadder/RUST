# Rust foundations

Build a study-log summarizer in four exercises. You will use tuples, borrowed strings, typed errors and iterators in one small Rust library.

| Lesson | Implement | Check |
| --- | --- | --- |
| [Divide a session into blocks](lessons/01-blocks.md) | `blocks` | `cargo test lesson1` |
| [Borrow a title](lessons/02-titles.md) | `title` | `cargo test lesson2` |
| [Parse a session](lessons/03-entries.md) | `parse_entry` | `cargo test lesson3` |
| [Project — summarize a study log](lessons/04-summary.md) | `summarize` | `cargo test lesson4` |

## Start

Install stable Rust and Cargo using [rustup](https://rustup.rs/), then run:

```sh
git clone https://github.com/SwarmLadder/RUST
cd RUST
cargo check --all-targets
cargo test lesson1
```

Open [the exercise file](src/exercises.rs). Each starter compiles and stops at an intentional `todo!()` until you implement it. Run the lesson's tests after each edit. Once all four functions work, run `cargo test` and `cargo run --example study_log`.

## Check a worked solution

`cargo test --features reference` runs the same tests against the worked solutions. `cargo run --example study_log --features reference` prints the completed report. Use the default commands when checking your own work.

This first module runs locally with Cargo and has no external crate dependencies. SwarmLadder Lab browser execution for Rust is a separate milestone.
