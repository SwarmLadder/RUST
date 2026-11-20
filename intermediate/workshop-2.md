# Keyed collections

Use maps for grouped values and sets for membership. The choice of collection determines whether iteration order is stable.

With a BTreeMap, `*counts.entry(key).or_insert(0) += 1` updates an existing count or creates it. A BTreeSet stores each distinct key once. Sorting by key and preserving input order are different contracts.

## Worked example

Predict the result before reading the assertions.

```rust
let mut counts = std::collections::BTreeMap::new();
for w in ["z", "a", "z"] { *counts.entry(w).or_insert(0) += 1; }
assert_eq!(counts.into_iter().collect::<Vec<_>>(), [("a", 1), ("z", 2)]);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `word_counts`

Count exact whitespace-delimited words with borrowed keys.

`cargo test --test intermediate word_counts -- --exact`

### `distinct`

Keep one copy of each value in increasing order.

`cargo test --test intermediate distinct -- --exact`

### `first_unique`

Return the earliest word in input order whose total frequency is one.

`cargo test --test intermediate first_unique -- --exact`

## Knowledge check

1. Which collection naturally yields sorted unique keys? Choose between BTreeSet or Vec.
2. How many distinct words are in 'Go go'? Choose between 1 or 2.
3. Where do borrowed word keys point? Choose between Into the source text or Into freshly cloned Strings.
4. How can you preserve the first appearance of unique items?
5. Why scan the original text again to find the first unique word?

<details><summary>Answer key and reasoning</summary>

1. BTreeSet. The tree maintains key order and suppresses duplicates.
2. 2. The contract uses exact matching.
3. Into the source text. Whitespace splitting yields slices of the input.
4. Keep a set for membership and append to a vector only on first insertion.
5. The map gives frequencies, but its key order differs from source order.

</details>
