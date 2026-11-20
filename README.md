# SwarmLadder Rust

Learn Rust locally with Cargo through **three levels, 36 guided workshops, 168 graded coding functions, nine portfolio projects, eight linked study-log lessons, and a dedicated engineering mastery track**. Each level includes 12 core workshops, 52 original coding exercises and three projects with acceptance tests and worked solutions; 12 more graded mastery functions extend the three levels. Existing workshop knowledge checks include explained answers.

| Level | What you will practice | What you will build |
| --- | --- | --- |
| [Basics](basics/README.md) | Values, conditions, UTF-8 text, slices and vectors | Trail elevation report, pantry shopping list, scoreboard |
| [Intermediate](intermediate/README.md) | Ownership, domain errors, collections, traits, ordering, data formats and closures | Invoice roll-up, concordance, room scheduler |
| [Advanced](advanced/README.md) | Borrowed views, streaming I/O, scoped threads and rollback | Event counter, parallel histogram, transactional ledger |

## New — Rust Engineering Mastery

The [Rust Engineering Mastery track](mastery/README.md) adds **12 additional graded engineering challenges** (4 per level) with starter `todo!()` code, reference implementations, acceptance tests and extensive study notes.

| Track | New challenges | What you learn |
|---|---|---|
| [Basics mastery](mastery/basics.md) | B01–B04 | Typed port parsing, Unicode scalar slicing, widened arithmetic, sorted word counts |
| [Intermediate mastery](mastery/intermediate.md) | I01–I04 | Precise parser diagnostics, borrowed indexes, interval normalization, deterministic scores |
| [Advanced mastery](mastery/advanced.md) | A01–A04 | Bounded binary I/O, scoped parallelism, transactional rollback, atomic compare-exchange |

**Start the new challenges:**

```sh
cargo check --all-targets
cargo test --test mastery b01_ports                     # Expected to fail until you solve B01
cargo test --features reference --test mastery          # Validate published answer key
cargo run --example mastery_demo --features reference   # Offline end-to-end demonstration
```

All mastery solutions use the **Rust standard library only**; no runtime database, cloud account, server or credentials are required. Student tests use the default Cargo features; the `reference` feature substitutes worked solutions to validate maintained course material—not your unfinished exercises.

For a structured end-to-end route, follow the [12-week curriculum](CURRICULUM.md), complete the [three mastery portfolio milestones](PORTFOLIO.md), and use the [100-point final assessment](ASSESSMENT.md). These materials are designed to be integrated by learners, but they make **no changes to SwarmLadderlab**.

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

## Full-course validation

The maintained curriculum and reference algorithms are checked by GitHub Actions. From the repository root run:

```sh
python3 tools/validate_course.py
cargo check --all-targets
cargo test --features reference
cargo doc --features reference --no-deps
```

**Learner-mode warning:** Existing course starter functions and the 12 new mastery starters deliberately contain `todo!()` placeholders. Their tests fail until you implement the requirements. Use `--features reference` only to verify course reference code, not as evidence of completing your own implementation.
