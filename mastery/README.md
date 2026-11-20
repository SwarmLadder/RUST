# Rust Engineering Mastery — 12 New Graded Challenges

This track extends, rather than replaces, SwarmLadder Rust's existing **36 guided workshops, 156 coding exercises, nine projects, and eight study-log lessons**. It adds **12 engineering-style functions and runnable test cases** covering Unicode boundaries, checked arithmetic, borrow-based APIs, deterministic data transformations, bounded I/O, scoped threading, atomic updates and compare-and-swap concurrency.

The mastery labs are **100% local and use only the Rust standard library**. They do not create GitHub accounts, require paid dependencies, run untrusted network services, or modify SwarmLadder Lab.

## How to use the mastery track

The exercise-facing API is in [starter.rs](starter.rs). It contains intentional `todo!()` placeholders. Reference algorithms live in [reference.rs](reference.rs), loaded only when the `reference` Cargo feature is enabled. Both modes use the same public error types in [types.rs](types.rs); integration tests in [../tests/mastery.rs](../tests/mastery.rs) enforce behavior.

```sh
# From the Rust repository root:
cargo check --all-targets
cargo test --test mastery b01_ports                   # Fails until you implement B01
cargo test --test mastery                              # Your own completion check
cargo test --features reference --test mastery         # Verify reference solutions
cargo test --features reference                        # Validate entire existing + mastery course
```

**Important:** The default mode intentionally fails unfinished tests. That is the expected learner experience. The reference feature tests solved implementations: it does **not** evaluate your own work, so do not use it to claim that your starter exercises are finished.

## Three levels

| Level | Four new mastery challenges | Skills and evidence |
|---|---|---|
| [Basics](basics.md) | B01 port parser · B02 Unicode window · B03 safe mean · B04 word frequency | Error contracts, UTF-8 scalar indexing, overflow, sorted maps |
| [Intermediate](intermediate.md) | I01 settings parser · I02 borrowed index · I03 interval normalization · I04 deterministic ranking | Error ordering, lifetimes/borrows, boundary-safe algorithms, stable policies |
| [Advanced](advanced.md) | A01 bounded binary frames · A02 parallel histogram · A03 atomic ledger changes · A04 capacity reservation | Streaming failure modes, scoped threads, transactional rollback, compare-exchange |

Read the challenge spec **before** reading its implementation. Every level guide includes a worked concept example, all four task contracts, a trace/debug scenario, boundary cases and an evidence checklist. Integrate each group of four in a [portfolio milestone](../PORTFOLIO.md) and the [final course assessment](../ASSESSMENT.md).

## Recommended sequence and pacing

Follow [the 12-week curriculum](../CURRICULUM.md). After Basics workshops 1–12, do B01–B04; after Intermediate workshops 1–12, do I01–I04; and after Advanced workshops 1–12, do A01–A04. Budget roughly 45–90 minutes per mastery task, plus review time.

### Reliable debugging loop

1. Read the full function contract and predict the type of its return value, ownership, and failure behavior.
2. Write one minimal success case and one edge case before implementing.
3. Run `cargo test --test mastery bXX_topic -- --nocapture` for your own implementation.
4. If it panics, locate the `todo!()` in starter.rs. If it fails an assertion, compare the exact expected and actual state.
5. Review `cargo test --features reference --test mastery` only to establish that the canonical implementation and tests agree.
6. Explain **why** the chosen Rust type prevents or reveals a class of mistakes (for example `Option`, `Result`, `&str`, `BTreeMap`, `AtomicUsize`).

## Boundary and safety contracts

- String windows count **Unicode scalar values**, not bytes or grapheme clusters. A visible character made from multiple scalars may occupy more than one index.
- Mean calculations widen to `i128` before adding `i64` inputs. Integer division truncates toward zero, including negative results.
- The word normalizer lowercases Unicode but removes **only ASCII punctuation at word edges**. It is not a full language-aware tokenizer or Unicode case-folding algorithm.
- Integer interval endpoints are **inclusive**; adjacency is merged even next to `u32::MAX`.
- The frame reader accepts a two-byte, big-endian payload length. Empty frame is valid; truncated data is an error. Limits are enforced before frame allocation.
- The histogram returns deterministic counts despite nondeterministic thread scheduling and caps worker creation at 16 threads.
- State updates must be all-or-nothing. Missing keys, underflow and overflow must return the index of the **first failing operation**.
- Atomic capacity reservations are lock-free compare-exchange exercises; they are **not** a distributed ticketing system, persistence layer or production access-control system.

## Completion checklist

- [ ] Four Basics mastery tests pass in default mode.
- [ ] Four Intermediate mastery tests pass in default mode.
- [ ] Four Advanced mastery tests pass in default mode.
- [ ] All 12 mastery tests pass together, without `reference`.
- [ ] Existing course tasks and projects still pass as you complete them.
- [ ] Three portfolio milestones and a final reflection report are prepared.
- [ ] Reference-mode CI and all content/link checks pass on the course repository.

**Do not copy the reference implementations as your first attempt.** Read them after recording your own tests, corner cases and design rationale.
