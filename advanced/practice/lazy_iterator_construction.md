# Lazy iterator construction

## Designing sequences on demand

Returning impl Iterator lets the caller choose how much to consume. A generator built with from_fn captures state between calls. Once exhausted, an iterator intended to be fused should stay exhausted.

**Worked trace.** A Fibonacci generator can yield 0 before calculating the next pair. Computing in a wider type or tracking an optional successor avoids losing the final representable term.

## Try it

```rust
let mut n = 1u32;
let odd = std::iter::from_fn(move || { let current = n; n += 2; Some(current) });
assert_eq!(odd.take(3).collect::<Vec<_>>(), [1, 3, 5]);
```

Move take before or after a filter and predict how many source elements each ordering consumes.

## Exercises

Edit [the starter functions](starter/lazy_iterator_construction.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `countdown`

Yield n down through zero, inclusive, lazily.

`cargo test --test advanced_practice countdown -- --exact`

### `stride`

Yield every step-th value from index zero. A zero step yields nothing.

`cargo test --test advanced_practice stride -- --exact`

### `prefixes`

Yield cumulative sums, widening before adding. Stop if the cumulative total overflows u64.

`cargo test --test advanced_practice prefixes -- --exact`

### `fib`

Yield 0,1,1,2,… through the final representable u32 Fibonacci value, then terminate.

`cargo test --test advanced_practice fib -- --exact`

### `runs`

Yield each consecutive byte and its run length without allocating run storage.

`cargo test --test advanced_practice runs -- --exact`

## Review questions

1. Why test countdown with take(1) on a huge input?
2. Why track the final Fibonacci value separately from its successor?

<details><summary>Check your reasoning</summary>

1. It checks that producing the first value needs no eagerly constructed sequence.
2. The successor may overflow while the current value is still representable.

</details>

[Compare worked solutions](solutions/lazy_iterator_construction.rs) · [Return to the level guide](../README.md)
