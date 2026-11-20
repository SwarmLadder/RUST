# Sharing and synchronization

## Coordinating independent workers

Arc shares ownership across threads. Mutex serializes access to protected state. An atomic read-modify-write can update one counter without losing concurrent increments. Pick a synchronization primitive that matches the invariant.

**Worked trace.** Two workers that each load a counter at 4 and later store 5 have lost an increment. fetch_add combines those operations into one atomic update.

## Try it

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
let next = AtomicUsize::new(4);
let first = next.fetch_add(1, Ordering::Relaxed);
let second = next.fetch_add(1, Ordering::Relaxed);
assert_eq!((first, second), (4, 5));
```

Replace fetch_add with a separate load and store on paper. Interleave two threads so both claim the same ticket.

## Exercises

Edit [the starter functions](starter/sharing_and_synchronization.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `share`

Return two handles to one allocation.

`cargo test --test advanced_practice share -- --exact`

### `ticket`

Atomically increment a standalone counter and return its previous value. Wrap on overflow.

`cargo test --test advanced_practice ticket -- --exact`

### `try_append`

Append only if try_lock succeeds immediately. Report contention or poison as false.

`cargo test --test advanced_practice try_append -- --exact`

### `parallel_max`

Use a scoped worker for one half and the caller for the other. Combine optional maxima.

`cargo test --test advanced_practice parallel_max -- --exact`

### `ordered_jobs`

Spawn each job in a scope, join all, preserve submission order and return the first panic in that order.

`cargo test --test advanced_practice ordered_jobs -- --exact`

## Review questions

1. When does try_lock make sense instead of lock?
2. Why collect every join result before returning the first panic?

<details><summary>Check your reasoning</summary>

1. When the caller can perform other work or decline the operation if the mutex is currently occupied.
2. Every scoped handle is explicitly joined, including later panics, so the scope does not rethrow an unhandled panic.

</details>

[Compare worked solutions](solutions/sharing_and_synchronization.rs) · [Return to the level guide](../README.md)
