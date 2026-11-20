# Reading sensor values

A thermometer reports whole degrees. Distinguish negative readings, count the cold ones, then handle a counter that might overflow.

Use `if` as an expression: `let label = if t < 0 { "cold" } else { "mild" };`. Both branches must produce the same type. For a counter with a fixed width, decide what should happen at its limit before choosing an operator.

## Worked example

Predict the result before reading the assertions.

```rust
let frost: Vec<_> = [-2, 0, 3].into_iter().map(|t| t <= 0).collect();
assert_eq!(frost, [true, true, false]);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `temperature`

Return cold below zero, otherwise mild.

`cargo test --test basics temperature -- --exact`

### `freezing`

Count readings at or below zero.

`cargo test --test basics freezing -- --exact`

### `tick`

Increase a counter by one, stopping at 255.

`cargo test --test basics tick -- --exact`

## Knowledge check

1. Which label does zero receive? Choose between mild or cold.
2. What is the count for an empty slice? Choose between 1 or 0.
3. Which operation keeps 255 at 255? Choose between saturating_add or wrapping_add.
4. Which two neighboring temperatures expose a mistaken `< 0` rule for frost?
5. How can a byte counter distinguish saturation from wrapping?

<details><summary>Answer key and reasoning</summary>

1. mild. The comparison is strictly less than zero.
2. 0. There are no matching elements.
3. saturating_add. Saturation clamps at the upper bound.
4. Zero and minus one. Both should count for a rule that includes freezing point.
5. Start at 255. Saturation keeps 255, wrapping produces zero.

</details>
