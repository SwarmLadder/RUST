# Integer arithmetic

## Building a clock display

Integer division discards the remainder. For elapsed time, the quotient gives whole units and the remainder gives the leftover part. Choose a wider type before multiplying values that might exhaust their input type.

**Worked trace.** For 3,661 seconds, divide by 3,600 first. The remaining 61 seconds hold one minute and one second.

## Try it

```rust
let seconds = 7_205u32;
let (hours, tail) = (seconds / 3_600, seconds % 3_600);
assert_eq!((hours, tail / 60, tail % 60), (2, 0, 5));
```

Change the input to 3,599. Which part rolls over after adding one second?

## Exercises

Edit [the starter functions](starter/integer_arithmetic.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `minutes`

Return whole elapsed minutes.

`cargo test --test basics_practice minutes -- --exact`

### `clock`

Split elapsed seconds into hours, minutes, seconds. Hours may exceed 23.

`cargo test --test basics_practice clock -- --exact`

### `area`

Calculate rectangular area without losing precision.

`cargo test --test basics_practice area -- --exact`

### `last_digit`

Return the final decimal digit, ignoring the sign.

`cargo test --test basics_practice last_digit -- --exact`

### `ceil_minutes`

Round partial minutes up, without overflowing.

`cargo test --test basics_practice ceil_minutes -- --exact`

## Review questions

1. Why widen before multiplying two u16 values?
2. What should rounding up an exact 120 seconds return?

<details><summary>Check your reasoning</summary>

1. The multiplication happens in the operand type. Converting its result is too late if that multiplication overflows.
2. Two minutes. Add one only for a nonzero remainder.

</details>

[Compare worked solutions](solutions/integer_arithmetic.rs) · [Return to the level guide](../README.md)
