# Enums and matching

## Operating a signal

An enum describes a closed set of possibilities. An exhaustive match forces a choice for every variant. When parsing external text, Option separates unrecognized input from every legal state.

**Worked trace.** A light that starts red becomes green, then amber. A blinking warning can be represented by a separate setting without adding a fourth color.

## Try it

```rust
use swarmladder_rust::basics_practice::Signal;
let signal = Signal::Amber;
let action = match signal { Signal::Red => "stop", Signal::Amber => "prepare", Signal::Green => "go" };
assert_eq!(action, "prepare");
```

Sketch the three transitions on paper. Which pair of variants is skipped when moving two steps from Green?

## Exercises

Edit [the starter functions](starter/enums_and_matching.rs). The [provided types](types.rs) define `Signal`. The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `next`

Cycle Red → Green → Amber → Red.

`cargo test --test basics_practice next -- --exact`

### `signal`

Recognize exactly red, green and amber in lowercase.

`cargo test --test basics_practice signal -- --exact`

### `may_go`

Permit movement only on Green.

`cargo test --test basics_practice may_go -- --exact`

### `advance`

Advance n steps without looping n times.

`cargo test --test basics_practice advance -- --exact`

### `green_count`

Count green signals in a trace.

`cargo test --test basics_practice green_count -- --exact`

## Review questions

1. How many steps restore every light to its original color?
2. Why return Option when parsing a signal?

<details><summary>Check your reasoning</summary>

1. Three steps, the length of the cycle.
2. Unknown text has no corresponding legal signal.

</details>

[Compare worked solutions](solutions/enums_and_matching.rs) · [Return to the level guide](../README.md)
