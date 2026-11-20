# 6. Accumulate minutes by topic

A weekly report needs one row per topic. Three sessions with durations 10, 5 and 20 can represent one, two or three topics. The title is the grouping key. Use `BTreeMap` so that iteration will later visit those keys in sorted order.

## Trace an update

Here is the same operation for counting votes:

```rust
use std::collections::BTreeMap;
let mut votes = BTreeMap::new();
for name in ["Jo", "Ana", "Jo"] {
    let count = votes.entry(name).or_insert(0);
    *count += 1;
}
assert_eq!(votes["Jo"], 2);
```

On the first Jo, `or_insert` creates a zero. On the second Jo, it returns a mutable reference to the existing count. Predict what the result would be if you replaced the update with `votes.insert(name, 1)` on every iteration.

## Implement totals

Borrow a slice of sessions. For each row, borrow its topic with `as_str()` and add the minutes to that topic’s total. The map borrows its keys from the input records, so the caller must keep the records alive while using it. Unlike a session in lesson 5, this map only needs to exist long enough to render the report.

Keep zero-minute topics. Match case exactly. Convert each duration with `u64::from` before adding it. Two individually valid u32 values can have a sum that exceeds u32.

| Session | Totals after the update |
| --- | --- |
| Rust,20 | Rust = 20 |
| rust,0 | Rust = 20; rust = 0 |
| Rust,7 | Rust = 27; rust = 0 |

Run `cargo test lesson6`. Explain why borrowing keys avoids a second copy of every title, and why a map borrowing from a local vector could not be returned after that vector is dropped.

[Next — render the report](07-render.md)
