# Ordering and binary search

## Looking up ranked records

Binary search relies on a sorted input contract. Equal values can occupy more than one slot. A partition point counts the prefix satisfying a monotone condition and can locate either end of an equal range.

**Worked trace.** In [1, 3, 3, 8], the first index with value at least 3 is 1. The first index with value greater than 3 is 3.

## Try it

```rust
let scores = [1, 3, 3, 8];
let start = scores.partition_point(|v| *v < 3);
let end = scores.partition_point(|v| *v <= 3);
assert_eq!(&scores[start..end], &[3, 3]);
```

Change the key to 5. Explain why the empty range still tells you something about insertion.

## Exercises

Edit [the starter functions](starter/ordering_and_binary_search.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `lower`

Given sorted input return the first index with value >= key, or len.

`cargo test --test intermediate_practice lower -- --exact`

### `equal_range`

Return the half-open equal range in sorted input, including the insertion position for absent keys.

`cargo test --test intermediate_practice equal_range -- --exact`

### `rank`

Sort descending by score, then ascending by name. Return names only.

`cargo test --test intermediate_practice rank -- --exact`

### `median`

Return the lower median of unsorted input without mutating it.

`cargo test --test intermediate_practice median -- --exact`

### `insert_unique`

Insert into sorted distinct input only if absent and report insertion.

`cargo test --test intermediate_practice insert_unique -- --exact`

## Review questions

1. Where is the insertion position for 4 in [1,3,3,8]?
2. Which item is the lower median in four sorted items?

<details><summary>Check your reasoning</summary>

1. Index 3, directly before 8.
2. The second item at index 1.

</details>

[Compare worked solutions](solutions/ordering_and_binary_search.rs) · [Return to the level guide](../README.md)
