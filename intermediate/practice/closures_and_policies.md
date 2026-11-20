# Closures and policies

## Making behavior configurable

A function taking FnMut permits callbacks to accumulate state. The order and number of calls can become part of the contract. Short-circuit a search as soon as its answer is known.

**Worked trace.** A validator that stops at the first bad record need never invoke the predicate on later records. A test callback can record exactly which items it saw.

## Try it

```rust
let mut inspected = Vec::new();
let found = [2, 4, 3, 8].into_iter().find(|v| { inspected.push(*v); v % 2 == 1 });
assert_eq!(found, Some(3));
assert_eq!(inspected, [2, 4, 3]);
```

Swap find for a filter followed by collect. Predict how the callback trace changes.

## Exercises

Edit [the starter functions](starter/closures_and_policies.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `first_bad`

Return index of the first failing element, evaluating only through that element.

`cargo test --test intermediate_practice first_bad -- --exact`

### `transform`

Move each element through the supplied transformation in source order.

`cargo test --test intermediate_practice transform -- --exact`

### `retry`

Call at most n times. Return first success, otherwise last failure. Zero attempts returns None.

`cargo test --test intermediate_practice retry -- --exact`

### `adjacent`

Check whether every adjacent pair satisfies agrees.

`cargo test --test intermediate_practice adjacent -- --exact`

### `fold_until`

Stop at the first None, returning the last successful accumulator.

`cargo test --test intermediate_practice fold_until -- --exact`

## Review questions

1. Why does retry accept FnMut?
2. Should first_bad evaluate the rest of the sequence after failure?

<details><summary>Check your reasoning</summary>

1. An attempt may update captured state between calls.
2. No. Stopping immediately preserves its callback contract and avoids extra side effects.

</details>

[Compare worked solutions](solutions/closures_and_policies.rs) · [Return to the level guide](../README.md)
