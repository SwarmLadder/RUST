# Loops and termination

## Tracing a repeated calculation

A loop needs a state, a condition for stopping, and a step that moves toward it. A zero input often skips the body entirely. Write its expected answer before choosing the initial accumulator.

**Worked trace.** For gcd(18, 12), the successive pairs are (12, 6) and (6, 0). The first component now holds the result.

## Try it

```rust
let (mut a, mut b) = (30, 18);
let mut divisors = Vec::new();
while b != 0 { divisors.push(b); (a, b) = (b, a % b); }
assert_eq!(divisors, [18, 12, 6]);
assert_eq!(a, 6);
```

Trace the loop with a smaller first operand. Write the next pair before calculating another remainder.

## Exercises

Edit [the starter functions](starter/loops_and_termination.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `gcd`

Find greatest common divisor by repeated remainder. Define gcd(0,0) as 0.

`cargo test --test basics_practice gcd -- --exact`

### `digits`

Count decimal digits, including the single digit in zero.

`cargo test --test basics_practice digits -- --exact`

### `factorial`

Multiply 1 through n with overflow detection.

`cargo test --test basics_practice factorial -- --exact`

### `powers`

List positive powers of two starting at 1 through limit, inclusively.

`cargo test --test basics_practice powers -- --exact`

### `digit_sum`

Sum decimal digits using division and remainder.

`cargo test --test basics_practice digit_sum -- --exact`

## Review questions

1. Why does the remainder loop terminate for gcd?
2. What starting accumulator makes zero factorial work?

<details><summary>Check your reasoning</summary>

1. Each nonzero divisor is followed by a strictly smaller nonnegative remainder.
2. One, the identity for multiplication.

</details>

[Compare worked solutions](solutions/loops_and_termination.rs) · [Return to the level guide](../README.md)
