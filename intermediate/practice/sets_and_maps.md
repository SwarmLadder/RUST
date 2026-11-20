# Sets and maps

## Reconciling inventories

Sets answer membership questions, while maps attach data to keys. Ordered collections produce deterministic reports. Treat duplicates deliberately when comparing stock lists.

**Worked trace.** Reconciling [3, 1, 1] with [2, 3] leaves one missing distinct ID, 1, even though it appeared twice.

## Try it

```rust
use std::collections::BTreeSet;
let ids = BTreeSet::from([7, 2, 7, 4]);
assert_eq!(ids.into_iter().collect::<Vec<_>>(), [2, 4, 7]);
```

What information about the raw inventory was discarded when the set was constructed?

## Exercises

Edit [the starter functions](starter/sets_and_maps.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `missing`

Return sorted distinct members of a absent from b.

`cargo test --test intermediate_practice missing -- --exact`

### `common`

Return sorted distinct common IDs.

`cargo test --test intermediate_practice common -- --exact`

### `invert`

Group names by value, retaining row order inside each bucket.

`cargo test --test intermediate_practice invert -- --exact`

### `duplicate`

Return the first ID encountered for the second time.

`cargo test --test intermediate_practice duplicate -- --exact`

### `vote`

Return the most frequent name, resolving ties by smallest lexicographic name.

`cargo test --test intermediate_practice vote -- --exact`

## Review questions

1. Why does a set difference drop duplicate rows?
2. What determines a tied vote winner?

<details><summary>Check your reasoning</summary>

1. Set membership records whether a value occurs, independently of frequency.
2. The lexicographically smallest name among the tied leaders.

</details>

[Compare worked solutions](solutions/sets_and_maps.rs) · [Return to the level guide](../README.md)
