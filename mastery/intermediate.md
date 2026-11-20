# Intermediate Mastery — Parsing, Borrowed Indexes and Deterministic Algorithms

**Four graded tasks:** I01–I04 · **Expected effort:** 5–7 hours · **Prerequisites:** [Intermediate workshops](../intermediate/README.md), ownership, borrowed views, `BTreeMap`, vectors, custom errors and sorting.

## Why this matters

Reliable applications depend on precisely specified transformations. Returning an error on the *right line*, borrowing text without leaking memory, merging intervals at maximum integer values, and breaking score ties deterministically are all more important than memorizing syntax.

## Worked example — hold borrowed data without copying it

A borrowed view can retain references to data owned by a caller:

```rust
fn nonblank<'a>(parts: &'a [String]) -> Vec<&'a str> {
    parts.iter()
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .collect()
}
let names = vec![String::from(" Ada "), String::from(" ")];
assert_eq!(nonblank(&names), vec!["Ada"]);
```

A borrow makes ownership clear. The returned slice references cannot outlive the input `Vec<String>`; the compiler enforces this instead of requiring manual resource tracking.

## I01 — Parse settings with precise error diagnostics

**Signature:** `parse_settings(text: &str) -> Result<BTreeMap<String, String>, ConfigError>`

**Contract:** Process physical lines in order, numbering them from 1. Ignore blank lines and lines whose **trimmed** content begins with `#`. For each remaining line, split at the *first* `=`; trim both key and value. An absent delimiter yields `MissingEquals(line)`. A blank key yields `EmptyKey(line)`. The first repeated key yields `DuplicateKey(line, key)`. Keys are **case-sensitive**; an empty value is allowed. Return a sorted map of parsed values if there are no errors.

**Trace:** `"# note\nmode = safe\n\nmode = fast"` → ignore line 1, accept mode on line 2, skip line 3, return `DuplicateKey(4, "mode")` on line 4.

**APIs:** `str::lines`, `enumerate`, `trim`, `split_once`, `?`, `BTreeMap::contains_key`.

**Failure clinic:** Splitting on *every* equals sign incorrectly rejects values such as `url=https://example`. Calling `lines().filter(...).enumerate()` after skipping blank lines loses physical line numbers. Accidentally inserting before checking duplicates destroys the first value.

**Test:** `cargo test --test mastery i01_settings`.

## I02 — Build a zero-copy first-character index

**Signature:** `by_initial(words: &[String]) -> BTreeMap<char, Vec<&str>>`

**Contract:** For every input string, take a trimmed borrowed view. Skip strings that become empty. Group the remaining views by their first Unicode scalar value, **without case normalization**. Preserve the input order of words inside each group; iterate keys in `char` sorted order thanks to `BTreeMap`. The returned `&str` slices are borrowed from the input—do not allocate new strings for them or return references to temporary values.

**Trace:** `[" apple ", " apricot", "éclair", ""]` → `'a' → ["apple", "apricot"]` and `'é' → ["éclair"]`.

**APIs:** `iter`, `trim`, `chars().next()`, `BTreeMap::entry`, and reference lifetime elision.

**Failure clinic:** Returning `&str` from a locally built `String` would cause a borrow-checker error, because the local owner is dropped at function exit. Fix ownership design rather than trying to bypass the lifetime model.

**Test:** `cargo test --test mastery i02_borrowed`.

## I03 — Normalize inclusive, unsorted intervals safely

**Signature:** `normalize_ranges(ranges: &[(u32,u32)]) -> Result<Vec<(u32,u32)>, usize>`

**Contract:** First scan input order and reject the earliest interval with start greater than end, returning its **zero-based** index. Otherwise, sort by start then end; merge intervals that overlap or touch (adjacent integer ranges). Return normalized sorted intervals. Empty input returns an empty vector. Handle `u32::MAX` without overflowing when testing adjacency.

**Trace:** `[(5,7),(1,2),(3,4),(10,10)]` → sort → `[(1,2),(3,4),(5,7),(10,10)]` → merge to `[(1,7),(10,10)]`.

**APIs:** `iter().enumerate()`, `sort_unstable`, `Vec::last_mut`, `saturating_add`, `max`.

**Failure clinic:** The naive comparison `start <= last_end + 1` can overflow when last_end is `u32::MAX`. In debug builds it may panic; in release semantics it can behave unexpectedly. Use explicit checked or saturating arithmetic for this boundary.

**Test:** `cargo test --test mastery i03_intervals`.

## I04 — Build a deterministic ranking policy

**Signature:** `top_k_scores(entries: &[(String,i64)], k: usize) -> Vec<(String,i64)>`

**Contract:** Return at most `k` items sorted by **descending score**, then **ascending name** for ties. Keep all duplicate input records as independent entries. If `k==0`, return an empty vector; if `k` exceeds the input size, return all entries. Do not mutate the caller's source slice. An empty input yields an empty result.

**Trace:** `[("Zed",4),("Bob",10),("Ada",10)]` with `k=2` → `[("Ada",10),("Bob",10)]`.

**APIs:** `to_vec`, `sort_by`, `Ord::cmp`, `Ordering::then_with`, `truncate`.

**Failure clinic:** Negating the score to reverse order is unsafe for `i64::MIN` because it cannot be negated as `i64`. Compare scores in reverse order directly. A `BTreeMap` keyed by name would incorrectly collapse duplicate records.

**Test:** `cargo test --test mastery i04_ranking`.

## Debugging and review practice

For the settings parser, write a table mapping each physical line to its outcome. For the borrowed index, sketch each `String` owner and the `&str` slices pointing into it. For interval normalization, trace one sequence that spans `u32::MAX`. For ranking, compare two equal scores and explain the tie-breaker, then test `i64::MIN`.

This level is about the *observable* policy of a function, not its particular implementation. Make your choice understandable to a caller and write assertions that defend boundary cases.

## Intermediate portfolio milestone

Design a small **configuration and ranking report** without requiring a web server. Provide four example config files (one clean, three rejected), a borrowed-name index demonstration, an unsorted set of scheduling windows and a ranked scoreboard. Include the expected output, the test run and a one-page explanation of why ownership, error ordering and determinism improve system reliability.

**Completion:** All four Intermediate mastery tests pass without `reference`, with at least one extra edge test of your own per task. Continue to [Advanced mastery](advanced.md).
