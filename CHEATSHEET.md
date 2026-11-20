# Rust Engineering Decision Guide

This quick reference summarizes patterns used across SwarmLadder's 36 workshops, 168 graded functions and 12 new mastery tasks. Read the [worked mastery lessons](mastery/README.md) for complete contracts and safety caveats.

## Ownership and borrowing

| Goal | Prefer | Caution |
|---|---|---|
| Return data borrowed from input | `fn trim(s: &str) -> &str` | Returned reference cannot outlive the input owner |
| Return independently owned data | `String`, `Vec<T>` | A clone allocates and may be unnecessary |
| Read without mutation | `&T`, `&[T]` | Shared immutable borrowing is usually simplest |
| Change caller-owned data | `&mut T` | Unique mutable borrow must be respected |
| Represent absence | `Option<T>` | Avoid magic sentinel values |
| Represent failure reason | `Result<T,E>` | Choose an error type with a useful policy |
| Group ordered keys | `BTreeMap<K,V>` | Deterministic key iteration, unlike a typical `HashMap` |
| Share across scoped workers | `std::thread::scope` | Workers may borrow stack-local data because they join before scope exit |

## Numeric and UTF-8 boundaries

| Risk | Pattern | Explanation |
|---|---|---|
| Narrow integer overflow | `i128::from(a) + i128::from(b)` | Widen **before** performing arithmetic |
| Overflow on next index | `index.checked_add(count)` | Can return `None` instead of panicking or wrapping |
| Endpoint adjacency near MAX | `end.saturating_add(1)` | Prevents overflow at `u32::MAX` |
| Check conversion into unsigned | `u64::try_from(value)` | Rejects negative and out-of-range values |
| UTF-8 scalar window | `str::char_indices()` | Converts scalar position to valid byte boundary |
| Whole-text iteration | `str::chars()` | Scalars are not necessarily user-perceived graphemes |
| Parse text into type | `str::parse::<T>()` | Still check the input grammar if your API disallows valid parser formats |
| Mean of signed inputs | `sum::<i128>() / (len as i128)` | Integer division truncates toward zero |

### UTF-8 example

```rust
let text = "Aé🙂Z";
assert_eq!(text.chars().count(), 4);
assert_eq!(text.len(), 8); // 1 + 2 + 4 + 1 bytes
assert_eq!(text.char_indices().map(|(i, _)| i).collect::<Vec<_>>(), [0, 1, 3, 7]);
```

Do not use `text[1..3]` to mean the second and third *characters*. It is a byte range and may not align with a requested scalar window.

## Collection and parsing decisions

| Goal | Common design |
|---|---|
| First error in a sequence | Iterate with `enumerate()` and return early with `?` |
| Physical line numbers | Enumerate **before** filtering blank or commented lines |
| Preserve input order in groups | Use `Vec` values in a `BTreeMap` |
| Deterministic top-k ranking | Sort by reversed numeric comparator, then ascending tie key |
| Prevent losing duplicate scores | Keep records in a vector rather than using a map keyed only by score |
| Merge inclusive intervals | Sort, then join overlapping or adjacent windows safely |
| Skip empty tokens | `split_whitespace()` and filter after normalization |

## I/O, concurrency and transactions

| Goal | Preferred primitive | Important limitation |
|---|---|---|
| Read exactly N bytes | `Read::read_exact` | Returns `UnexpectedEof` for truncated input |
| Detect clean EOF at frame boundary | Read first header byte and check 0 | Partial header must still be rejected |
| Reject oversized payloads | Check claimed length **before allocating** | Length-prefix alone is not a secure protocol |
| Parallel counting | Scoped worker-local arrays, then deterministic merge | Limit worker count; spawning more is not always faster |
| All-or-nothing in-memory batch | Modify a validated snapshot and replace original after success | Not durable against process crashes |
| Bound concurrent count | CAS loop with `checked_add` and capacity comparison | Only process-local synchronization |
| Publish a failed operation | `Result<_,Error>` with no state mutation | Caller still needs a recovery/retry policy |

### Compare-exchange rule of thumb

A `load` followed by `store` is **not** enough to reserve a scarce resource concurrently: two threads may both read the same old value. A CAS operation commits only when the value still matches the observed value, and must be retried on contention. Use ordering appropriate for the actual data being synchronized; a working counter example is not a substitute for a complete memory-model review.

## Commands

```sh
cargo check --all-targets                           # Compile all starter and test targets
cargo test --test mastery b01_ports                  # Student code; fails until solved
cargo test --features reference --test mastery       # Validate canonical answers only
cargo test --features reference                      # Validate the complete maintained course
cargo run --example mastery_demo --features reference
cargo doc --features reference --no-deps
python3 tools/validate_course.py                     # Curriculum and link structure
```

All new mastery problems use only the standard library. For the [12-week plan](CURRICULUM.md) and grading standards see [the assessment](ASSESSMENT.md).
