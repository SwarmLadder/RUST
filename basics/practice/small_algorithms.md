# Small algorithms

## Checking patterns in measurements

Windows expose neighbors without indexing past the end. A running streak needs both its current length and the best length seen. Keep those two roles distinct when a mismatch resets one of them.

**Worked trace.** The trace [1, 1, 2, 1] has three matching samples but its longest consecutive run of ones is two.

## Try it

```rust
let trace = [1, 1, 0, 1];
let transitions: Vec<_> = trace.windows(2).map(|p| (p[0], p[1])).collect();
assert_eq!(transitions, [(1, 1), (1, 0), (0, 1)]);
```

Delete the first element. Which pairs survive, and what does that do to the longest streak?

## Exercises

Edit [the starter functions](starter/small_algorithms.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `sorted`

Accept nondecreasing input, including empty slices.

`cargo test --test basics_practice sorted -- --exact`

### `streak`

Find the longest consecutive run of key.

`cargo test --test basics_practice streak -- --exact`

### `changes`

Count adjacent unequal pairs.

`cargo test --test basics_practice changes -- --exact`

### `palindrome`

Check scalar symmetry, including spaces and case.

`cargo test --test basics_practice palindrome -- --exact`

### `merge_pairs`

Interleave elements starting with a, then append the longer remainder.

`cargo test --test basics_practice merge_pairs -- --exact`

## Review questions

1. What are the two streak counters after scanning [1,1,0] for 1?
2. Why does an empty sequence satisfy nondecreasing order?

<details><summary>Check your reasoning</summary>

1. Current length is zero. Best length is two.
2. There is no adjacent pair that violates the condition.

</details>

[Compare worked solutions](solutions/small_algorithms.rs) · [Return to the level guide](../README.md)
