# Building collections

Build new vectors by selecting and transforming input. Preserve the order of items unless the contract says to sort them.

`xs.iter().copied()` yields owned integer values. `filter` selects a subset, `map` transforms items, and `collect` creates the output collection. Iterators are lazy until consumed.

## Worked example

Predict the result before reading the assertions.

```rust
let v: Vec<_> = [3, 1, 4].windows(2).map(|p| p[1] - p[0]).collect();
assert_eq!(v, [-2, 3]);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `positives`

Keep strictly positive readings in order.

`cargo test --test basics positives -- --exact`

### `lengths`

Return scalar counts in the original order.

`cargo test --test basics lengths -- --exact`

### `pairs`

Return adjacent overlapping pairs.

`cargo test --test basics pairs -- --exact`

## Knowledge check

1. Which adaptor can change output length by dropping items? Choose between filter or map.
2. How many adjacent pairs are in four items? Choose between 2 or 3.
3. When is a mapped iterator evaluated? Choose between When consumed or When constructed.
4. How many overlapping pairs come from four items?
5. Does collecting positive values require sorting?

<details><summary>Answer key and reasoning</summary>

1. filter. A filter only emits matches.
2. 3. Overlapping windows start at positions zero, one and two.
3. When consumed. Creating the iterator describes a computation.
4. Three. The second item ends the first pair and starts the second.
5. No. Filtering preserves traversal order, including repeated values.

</details>
