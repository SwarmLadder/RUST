# Rust Final Assessment — Engineering Competence

This is an **evidence-based rubric** for the complete Rust course, not an automatic certificate. The library's reference feature validates the course's solution key; it cannot prove that the student has completed their own exercises.

## Eligibility to submit

- Review all **36 level workshops** and explain their core concepts.
- Implement and pass all **156 existing level practice functions** and **12 mastery functions** in default learner mode.
- Complete the **nine level projects** and **eight linked study-log lessons**.
- Submit the **three new mastery portfolio milestones** with real command outputs.
- Provide at least one regression/edge-case test of your own for each mastery level.

## Practical assessment — 100 points

| Dimension | Points | Required evidence |
|---|---:|---|
| Rust ownership and borrowing | 20 | A safe borrowed-view design; clear explanation of lifetimes, moves and aliasing |
| Types and error semantics | 15 | Correct `Result`/`Option` contracts and first-failing error location |
| Algorithms and boundary handling | 20 | Unicode scalar boundaries, checked arithmetic, deterministic intervals and ranking |
| Systems I/O and resource limits | 15 | Length-prefixed frames, clean EOF vs truncation, resource bounds |
| Concurrency and atomic state | 15 | Scoped worker histogram, correct CAS under races, bounded thread count |
| Tests, maintainability and reasoning | 15 | Focused, honest test output, regression checks, reproducible build, review notes |

**Pass threshold:** 80/100 plus all mandatory safety gates. A score of 80 with a serious unaddressed race or failed rollback does **not** pass.

## Mandatory correctness and safety gates

1. No unsafe Rust blocks, fabricated test runs or copy-pasted solution key submitted as original work.
2. No panic on valid but unusual UTF-8 input; no unchecked addition on boundary cases requiring safe arithmetic.
3. No partial state update after a rejected transaction. Missing key, overflow and underflow must leave all balances unchanged.
4. No oversubscription under concurrent reservations. The active count never exceeds limit, including under races.
5. Binary input processing must reject truncated payloads and enforce bounds **before allocation**.
6. No claim that process-local atomics or an in-memory map are a production-safe distributed booking, banking or authentication solution.
7. Every project claim must include a reproducible command, its actual result, or an explicit **simulation** label.

## Exercise and project scoring

For each mastery level, graders should consider:
- **Correctness (45%)** — tests pass and contract behavior matches all specified edge cases.
- **Safety and reliability (25%)** — no unchecked panic/overflow, data race, invalid borrow or partial commit.
- **Rust design quality (20%)** — good ownership model, coherent error types, clear use of standard library primitives.
- **Documentation (10%)** — concise intent, exact reproducible commands, limitations and recovery path.

## Oral / code-review prompts

**Basics:** Why can UTF-8 byte indexing panic? Why does widening a sum after doing the addition fail? Explain the difference between `Some("")` and `None` in a character window.

**Intermediate:** Show why a function cannot return `&str` referencing a temporary `String`. Why do first-error line numbers need to be counted before skipping comments? How do you merge ranges adjacent to `u32::MAX`? What is the danger of computing descending rank by negating `i64::MIN`?

**Advanced:** Show all cases where a binary frame reader must return `UnexpectedEof` versus `InvalidData`. Explain how scoped threads borrow source slices safely. Demonstrate atomic rollback of a two-step ledger. Sketch two threads racing to reserve the final slot and the required compare-exchange retry.

## Test commands and evidence

```sh
cargo check --all-targets
cargo test --test mastery b01_ports
cargo test --test mastery i01_settings
cargo test --test mastery a04_atomic
cargo test --test mastery
cargo test --features reference
cargo doc --features reference --no-deps
```

Default-mode tests that still reach `todo!()` are **not** completed. The reference-mode command is for verifying maintained answer keys, and should be reported separately from learner results. Record the Rust toolchain version with `rustc --version`.

## Final self-review template

1. **Problem:** Identify which behavior the function promises and the risk of violating it.
2. **Design:** Explain types, ownership, error ordering, invariants and resource budget.
3. **Implementation:** Point to the student's own code and tests.
4. **Evidence:** Provide exact commands, observed outputs and one adversarial boundary test.
5. **Limitations:** Explain which security/reliability claims remain outside the exercise's scope.
6. **Next action:** State a follow-up improvement if it were to become a real product.

The [12-week curriculum](CURRICULUM.md) and [portfolio requirements](PORTFOLIO.md) provide the pacing and submission checklist.
