# Slices and searching

A queue of readings may be empty. Use Option to express that a requested value was absent.

Iterate with `for (i, value) in xs.iter().enumerate()` to keep a zero-based position. `xs.first().copied()` turns a borrowed element into a copied value when the element implements Copy.

## Worked example

Predict the result before reading the assertions.

```rust
let queue = [8, 5, 8];
assert_eq!(queue.iter().rposition(|&v| v == 8), Some(2));
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `first_even`

Find the first even value in input order.

`cargo test --test basics first_even -- --exact`

### `peak`

Return the largest reading, including negative values.

`cargo test --test basics peak -- --exact`

### `position_of`

Return the first matching zero-based index.

`cargo test --test basics position_of -- --exact`

## Knowledge check

1. Why is zero a bad default maximum? Choose between All readings might be negative or Zero cannot be stored.
2. Which match should position return for duplicates? Choose between Last or First.
3. Does borrowing a slice consume the vector? Choose between No or Yes.
4. Why is zero a poor sentinel for a missing index?
5. How would a last-occurrence requirement change the search?

<details><summary>Answer key and reasoning</summary>

1. All readings might be negative. A made-up starting value can exceed the real maximum.
2. First. The iterator stops at the first match.
3. No. A slice is a view of existing storage.
4. Zero is a valid first index. None expresses absence independently.
5. Scan from the back or track the latest match until traversal ends.

</details>
