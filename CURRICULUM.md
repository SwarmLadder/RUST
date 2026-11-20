# SwarmLadder Rust — 12-Week Complete Learning Roadmap

**Format:** Self-paced, local-first Rust engineering. Use stable Rust and Cargo, a terminal, an editor, and a disposable Git branch. No paid API, cloud account or third-party Rust crate is required. Rust 2021 edition is used in this repository.

This course already contains **36 detailed workshops**, **156 existing practice functions** (52 per level), **9 scored projects**, and **8 linked study-log lessons**. The engineering mastery extension adds **12 more graded functions**, **three level-specific application briefs**, and a unified, evidence-based final assessment.

## Course learning objectives

By the end, a learner should be able to explain the borrow checker rather than merely silence it; define precise types for success and failure; select correct numeric and UTF-8 boundaries; build deterministic transformations; test and refactor a Cargo application; handle binary I/O, thread scheduling and atomic state; and distinguish an educational in-memory transaction from durable real-world infrastructure.

## Weeks 1–4 — Rust Basics

| Week | Workshop work | Practice evidence | Milestone |
|---|---|---|---|
| 1 | [Basics workshops 1–3](basics/README.md): variables, Unicode, slices | First 12 level tests; explain scalar vs byte | Rust toolchain and test workflow |
| 2 | Basics workshops 4–6: collections, integer arithmetic, decisions | Five exercise cases per chapter; inspect `Option` and `Result` | Collection and branch patterns |
| 3 | Basics workshops 7–9: loops, tuples, strings | Unit tests for numeric boundaries and UTF-8 | Trace a loop and explain ownership |
| 4 | Basics workshops 10–12: borrowing, enums and algorithms | All 52 Basics exercise functions, [three Basics projects](basics/README.md), mastery B01–B04 | [Basics mastery](mastery/basics.md) portfolio evidence |

**Exit criterion:** Pass the regular Basics tests without the reference feature; solve B01–B04; explain why you cannot index a Rust `str` by arbitrary bytes.

## Weeks 5–8 — Intermediate Rust

| Week | Workshop work | Practice evidence | Milestone |
|---|---|---|---|
| 5 | [Intermediate workshops 1–3](intermediate/README.md): typed errors, maps, generic functions | Tests for error and collection behaviors | Robust validation patterns |
| 6 | Intermediate workshops 4–6: iterators, ownership, custom errors | Compare borrowed and owned interfaces | API design critique |
| 7 | Intermediate workshops 7–9: maps, queues, traits | Deterministic and maintainable collections | Ordered data presentation |
| 8 | Intermediate workshops 10–12: ordering, delimited data and policies | All 52 Intermediate exercise functions, [three Intermediate projects](intermediate/README.md), mastery I01–I04 | [Intermediate mastery](mastery/intermediate.md) portfolio evidence |

**Exit criterion:** Pass regular Intermediate tests without reference; solve I01–I04; trace a borrowed API and explain its source lifetime.

## Weeks 9–12 — Advanced Rust

| Week | Workshop work | Practice evidence | Milestone |
|---|---|---|---|
| 9 | [Advanced workshops 1–3](advanced/README.md): lifetimes, streams, concurrency | Demonstrate error propagation and scoped worker behavior | Trace thread lifetimes |
| 10 | Advanced workshops 4–6: atomic updates, iterators, recursive structures | Check rollback, iteration and recursion | Safe refactoring |
| 11 | Advanced workshops 7–9: graphs, binary framing, synchronization | Test malformed input and worst-case boundaries | Reliability report |
| 12 | Advanced workshops 10–12: generic streams, versioning and concurrency | All 52 Advanced exercise functions, [three Advanced projects](advanced/README.md), mastery A01–A04, final assessment | [Advanced mastery](mastery/advanced.md) and [final portfolio](PORTFOLIO.md) |

**Exit criterion:** Pass the regular Advanced tests without reference; solve A01–A04; demonstrate that malformed frames, partial transactions and simultaneous capacity requests fail safely.

## Eight linked study-log lessons

The [original eight lessons](README.md#guided-study-log-project) can run alongside the three levels. They build a study-log application incrementally: four foundations lessons (blocks, title borrowing, parsing, summary) and four higher-level lessons (owned sessions, totals, rendering and CLI output). Finish them as an additional integration project.

## Repeatable study session

1. **25 minutes:** Read the workshop text, diagram ownership and predict a result before looking at a reference.
2. **35 minutes:** Complete several starter functions or project steps in a local disposable branch.
3. **20 minutes:** Run focused Cargo tests, debug one edge case, and add a regression assertion.
4. **10 minutes:** Document the failure cause, chosen API contract and test evidence.

Target **7–10 hours/week**. Take longer where needed. The calendar is a pacing guide, not a hard dependency.

## Testing: learner mode vs reference mode

```sh
# Compile all starter-mode code (does not execute todo!() functions):
cargo check --all-targets

# Run only a function you have implemented:
cargo test --test mastery b01_ports

# Use level-specific tests to validate your own existing course exercises:
cargo test --test basics --test basics_practice --test basics_projects

# After your own solutions pass, verify the published reference key:
cargo test --features reference

# Compile documentation and validate the course packaging:
cargo doc --features reference --no-deps
```

**Why the separation matters:** Default-mode exercise functions intentionally contain `todo!()`. An uncompleted learner checkout is supposed to fail the corresponding execution tests, which make good practice checkpoints. Reference mode substitutes worked solutions to test correctness of the course material itself. Passing a reference test does not certify the learner's own code.

## Required completion evidence

- All **36 workshops** studied and their tracing/review questions answered.
- All **156 original exercise functions** implemented and checked in learner mode.
- All **12 new mastery functions** implemented and their tests checked in learner mode.
- **Nine original course projects**, the **eight-lesson study log**, and [three mastery portfolio milestones](PORTFOLIO.md) delivered with evidence.
- One full [100-point final assessment](ASSESSMENT.md) at a score of 80+, meeting the safety and reliability gates.
- At least one original boundary/regression test for each mastery level.

The repository does not collect student data or track completion on a server. Record your own results in a local evidence journal.

## Common failure modes

- **A test panics at todo!():** Implement the corresponding function in the starter file; do not enable reference mode as a workaround.
- **Borrow-checker rejects a return value:** Trace where the value is owned, and whether your function returns a reference to a temporary.
- **A value overflows in debug mode:** Verify every intermediate arithmetic type, not only the type of the final result.
- **An async or threaded demo behaves differently across runs:** Check what order the API actually promises. Scheduling is not deterministic.
- **A performance improvement adds state races:** Prefer immutable input splitting or a tested atomic/lock pattern, then compare correctness before speed.

For capstone review, use the [portfolio submission standards](PORTFOLIO.md) and [final assessment rubric](ASSESSMENT.md).
