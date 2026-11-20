# Rust Engineering Portfolio — Three Mastery Milestones

These three applied briefs combine the new 12 mastery challenges with the existing workshop curriculum. They are designed as reproducible local code-review artifacts, **not** hosted deployments. The objective is to demonstrate that the student can reason about ownership, failure modes, concurrency and testing.

You may implement each brief as a small standalone Rust program in a **separate local directory** depending on your version of the course library, or present a clear test-driven technical report. Do not modify existing starter test contracts merely to obtain a passing result.

## Milestone 1 — Input diagnostics library (Basics)

**Client story:** A command-line importer receives messy text containing network ports, international names, numeric readings and labels. The importer must clearly distinguish missing data from malformed data, and must not crash on Unicode or large integer values.

**Use:** B01 (ports), B02 (Unicode windows), B03 (wide mean), B04 (word frequency).

### Acceptance tasks

- Process a list of at least eight port candidates, including whitespace, zero, maximum port, too-large value, sign, punctuation and non-ASCII digits. Show the exact error enum or accepted value.
- Demonstrate a borrowed window into a UTF-8 string containing at least one multi-byte scalar and a zero-length window at the end.
- Calculate means from a positive list, negative list, empty list and a list containing extreme `i64` values without narrow intermediate overflow.
- Generate a sorted word-frequency listing from mixed-case and Unicode text, with internal punctuation preserved.

**Deliverable:** A short sample program or Markdown test notebook, input/output examples, and one paragraph explaining each public return type.

**Critical review question:** Which problems are compile-time ownership problems, and which are runtime input-validation problems? Where must the function return `None` or `Err` instead of panicking?

## Milestone 2 — Typed configuration and scheduling report (Intermediate)

**Client story:** A small internal tool reads repeated key/value configuration lines, groups report labels by first scalar, consolidates reserved time windows and publishes the highest scores deterministically.

**Use:** I01 (settings), I02 (borrowed index), I03 (intervals), I04 (ranking).

### Acceptance tasks

- Parse a clean config and three rejected configs: missing delimiter, blank key, duplicate key. Preserve physical line numbers even when comments and empty lines occur.
- Build an index of owned `String` values without copying the original word contents. Draw the relationship between source lifetime and returned `&str` references.
- Merge out-of-order overlapping and adjacent inclusive ranges; test `u32::MAX` and a range whose start exceeds its end.
- Produce a deterministic top-five score ranking, including tie scores, duplicated names, `i64::MIN`, and fewer than five input records.

**Deliverable:** Demonstration, unit test transcript, deterministic sample output and a brief API-design comparison of using `BTreeMap` versus `HashMap` and owned versus borrowed strings.

**Critical review question:** Why does Rust's lifetime tracking prevent the borrowed index from outliving its owning inputs, while a data-format error still needs explicit runtime validation?

## Milestone 3 — Reliable in-memory service kernel (Advanced)

**Client story:** A process receives bounded binary frames, counts byte frequencies across workers, applies a batch of account deltas and allocates a limited number of in-process slots. Each operation must fail without invalid partial state.

**Use:** A01 (frame decoding), A02 (parallel histogram), A03 (transactional update), A04 (atomic reservation).

### Acceptance tasks

- Decode a valid zero-length frame and a nonempty frame. Reject an incomplete length header, incomplete payload, too-large payload and additional frame beyond the allowed count.
- Compare serial and multithreaded histograms over the same 5,000-byte sample and prove both sum to the input length.
- Show the before/after map for a valid batch, then demonstrate that a later failing delta does **not** retain earlier successful deltas.
- Run at least 32 competing threads against a limit smaller than 32; count the number of successful reservations and verify the final atomic count equals that number and never exceeds the limit.
- Record toolchain version, test command and any errors you corrected during implementation.

**Deliverable:** Minimal source/test code, failure table, trace for a two-thread race, and a design-review section explaining *what this exercise is not*: it is not durable, distributed, authenticated or crash-recoverable.

**Critical review question:** What is the difference between a process-local atomic counter and an actual reservation system that survives restarts and coordinates across multiple machines?

## Evidence journal template

Use the following format for **each** milestone:

```markdown
# Milestone title and Rust toolchain
Problem statement:
Functions used:
Input and expected result:
Output observed:
Test commands and status:
One intentionally failing edge case:
Ownership / error / concurrency reasoning:
What still needs validation before real-world use:
```

Do not paste genuine secrets or real-user data in samples. Label test data as synthetic.

## Evaluation

| Area | Weight |
|---|---:|
| Behavioral correctness and boundary coverage | 40% |
| Rust API and ownership reasoning | 25% |
| Reliability, resource bounds and safe failure | 20% |
| Documentation and reproducibility | 15% |

A milestone is complete when it includes the specified evidence and the corresponding default-mode mastery tests pass on the student's implementation. Reference-mode tests establish that the published reference answer remains valid, not that the student's own code is complete.

For final grading use [the 100-point assessment](ASSESSMENT.md). For weekly pacing see [the curriculum](CURRICULUM.md).
