# Basics Mastery — Correct Types, UTF-8 and Safe Arithmetic

**Four graded tasks:** B01–B04 · **Expected effort:** 4–6 hours · **Prerequisites:** [Basics workshops](../basics/README.md), `Option`, `Result`, slices, references, collections and Cargo tests.

## Why this matters

A small Rust program can compile and still get correctness wrong at its boundaries. These challenges exercise the reasoning that makes Rust API design useful: reject ambiguous inputs, never slice a UTF-8 string at arbitrary byte positions, account for numeric overflow, and specify deterministic output.

Start from [the exercise-facing code](starter.rs), not [the solutions](reference.rs). In your own code, prefer explicit contracts and straightforward algorithms over clever one-liners.

## Worked example — use a type to represent possible failure

A temperature sensor may or may not have an available value. Instead of returning a magic number, express the absence explicitly:

```rust
fn first_reading(readings: &[i32]) -> Option<i32> {
    readings.first().copied()
}
assert_eq!(first_reading(&[3, 5]), Some(3));
assert_eq!(first_reading(&[]), None);
```

This same discipline applies to B01: different invalid inputs have different meanings, so use a typed `Result<u16, PortError>` rather than pretending every failure is just zero.

## B01 — Parse a safe nonzero network port

**Signature:** `parse_port(input: &str) -> Result<u16, PortError>`

**Contract:** Remove surrounding Unicode whitespace first. Blank text returns `PortError::Empty`. Any remaining non-ASCII digit yields `InvalidCharacters`, including sign symbols, decimal points and non-ASCII digit shapes. A number outside the inclusive range 1–65535 yields `OutOfRange`, including "0" or numerals larger than `u16::MAX`. Leading zeros are accepted. Return the parsed port on success.

**Trace:** `" 00443 "` → trimmed `"00443"` → every byte an ASCII digit → parse as `u16` → `Ok(443)`.

**Useful Rust APIs:** `str::trim`, `str::bytes`, `u8::is_ascii_digit`, `str::parse`, `match`.

**Common mistake:** Calling `parse` before rejecting signs permits formats the contract disallows; mapping all parse failures to one error loses diagnostic precision.

**Test:** `cargo test --test mastery b01_ports`.

## B02 — Borrow a Unicode-safe character window

**Signature:** `unicode_window(input: &str, start: usize, count: usize) -> Option<&str>`

**Contract:** Interpret `start` and `count` as numbers of **Unicode scalar values** (`char`), not UTF-8 bytes. Return a borrowed substring containing exactly `count` scalars starting at scalar index `start`. If the range is beyond the text or its index arithmetic overflows, return `None`. A zero-length window at the end of the string returns `Some("")`. An empty window beyond the end returns `None`.

**Trace:** `"Aé🙂Z"` has four scalars occupying 1+2+4+1 UTF-8 bytes. Window (1, 2) returns `"é🙂"`; slicing bytes using `[1..3]` would be invalid for this requested scalar window.

**Useful Rust APIs:** `char_indices`, `checked_add` and borrowed `&str` ranges at valid byte boundaries.

**Common mistake:** Calling `input[start..start + count]` for Unicode indices may panic at non-boundary byte offsets; using `chars().nth` without translating back to bytes cannot slice the original string.

**Test:** `cargo test --test mastery b02_utf8`.

## B03 — Compute an integer mean without intermediate overflow

**Signature:** `mean_truncated(values: &[i64]) -> Option<i64>`

**Contract:** Empty input has no mean (`None`). Otherwise, compute sum/length using a *widened* `i128` accumulator before division. Integer division truncates toward zero. The mean of finitely many `i64` values is always representable in `i64`; do not lose it through early narrow summation. Examples: [1,2] → 1; [-1,-2] → -1; [MAX,MAX] → MAX.

**Trace:** `i64::MAX + i64::MAX` overflows `i64`, but widening both operands to `i128` first preserves their sum.

**Useful Rust APIs:** `iter`, `map`, `i128::from`, `sum` and integer division.

**Common mistake:** Performing the `i64` addition first and only widening afterward; once overflow occurs, the conversion is too late.

**Test:** `cargo test --test mastery b03_means`.

## B04 — Count normalized words deterministically

**Signature:** `word_frequency(text: &str) -> BTreeMap<String, usize>`

**Contract:** Split on Unicode whitespace; trim **ASCII punctuation from the beginning and end** of each token; lowercase the remainder using Unicode lowercase conversion; skip empty results and count duplicates. Keep punctuation *inside* a token. A `BTreeMap` yields sorted key iteration. This is an educational token policy, not linguistic case folding or Unicode grapheme segmentation.

**Trace:** `"Rust! RUST co-op ... é É"` → `rust` twice, `co-op` once, `é` twice; the isolated punctuation token is ignored.

**Useful Rust APIs:** `split_whitespace`, `trim_matches`, `to_lowercase`, `BTreeMap::entry`.

**Common mistake:** Using `split(' ')` fails with tabs/newlines and repeated spaces. Lowercasing only ASCII letters misses Rust's Unicode-aware conversion. Splitting on all punctuation changes the meaning of internal hyphens.

**Test:** `cargo test --test mastery b04_words`.

## Debugging practice

For each task, create a small table with your predicted result before running the test. Check not only the normal case but also one empty input, one Unicode input, and one maximum/boundary value. Explain what the return type communicates to the caller. For B02, draw the scalar positions and byte offsets separately; for B03, write down the type of the sum at every step.

## Basics portfolio milestone

Build a **local input diagnostics sheet** in Markdown containing sample input/output for B01, B02, B03 and B04. Include at least eight boundary cases. Explain why a Rust API with `Result` or `Option` is preferable to returning a misleading default value, and show the exact command used to pass each test.

**Completion:** All four default-feature Basics mastery tests pass; you can explain UTF-8 and overflow pitfalls without reading the reference source. Continue to [Intermediate mastery](intermediate.md).
