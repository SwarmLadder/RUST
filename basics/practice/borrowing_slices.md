# Borrowing slices

## Manipulating a sequence in place

A mutable slice gives temporary exclusive access to existing elements. After its borrow ends, the caller sees their new values. Compare two states in your tests: the return value and the contents of the original slice.

**Worked trace.** Splitting [4, 8, 9] at index 1 gives [4] and [8, 9]. The boundary belongs to the second slice.

## Try it

```rust
let mut data = [4, 8, 9];
let (left, right) = data.split_at_mut(1);
left[0] += right[0];
assert_eq!(data, [12, 8, 9]);
```

Try the split at the array length. Which of the two index expressions would become invalid?

## Exercises

Edit [the starter functions](starter/borrowing_slices.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `ends`

Copy the first and last values when both exist.

`cargo test --test basics_practice ends -- --exact`

### `suffix`

Borrow the final n items, or the entire slice when shorter.

`cargo test --test basics_practice suffix -- --exact`

### `split_at_checked`

Split at n when n is at most the length.

`cargo test --test basics_practice split_at_checked -- --exact`

### `swap_ends`

Swap first and last only when there are at least two elements. Report whether a swap occurred.

`cargo test --test basics_practice swap_ends -- --exact`

### `shift`

Rotate nonempty input left by n positions modulo its length. Empty input stays empty.

`cargo test --test basics_practice shift -- --exact`

## Review questions

1. What suffix should a zero count return?
2. Why handle an empty rotation before taking a remainder?

<details><summary>Check your reasoning</summary>

1. An empty slice at the end of the input.
2. A length of zero would be a zero divisor.

</details>

[Compare worked solutions](solutions/borrowing_slices.rs) · [Return to the level guide](../README.md)
