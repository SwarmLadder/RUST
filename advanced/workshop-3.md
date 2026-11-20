# Concurrency and message passing

Split immutable input across scoped threads. Return owned partial results and combine them in a deterministic order.

A scoped thread can borrow stack data because the scope waits for its threads before exit. Joining handles in submission order keeps results in that order even when workers finish at different times. Scheduling does not guarantee which worker finishes first.

## Worked example

Predict the result before reading the assertions.

```rust
let values = [3, 5];
let result = std::thread::scope(|s| s.spawn(|| values[0] * values[1]).join().unwrap());
assert_eq!(result, 15);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `chunk_size`

Return ceiling division, including zero items. Reject zero workers; avoid overflow.

`cargo test --test advanced chunk_size -- --exact`

### `parallel_sum`

Split in half, sum one half on a scoped thread and the other on the caller, widening each item.

`cargo test --test advanced parallel_sum -- --exact`

### `drain`

Collect all messages until every sender disconnects, preserving receive order.

`cargo test --test advanced drain -- --exact`

## Knowledge check

1. Why does this receiver loop terminate? Choose between Every sender is dropped or The first message was received.
2. What allows a scoped worker to borrow a slice? Choose between Slices have static storage or Its lifetime is bounded by the scope.
3. Why avoid (items + workers - 1) / workers? Choose between The addition may overflow or Integer division rounds up.
4. A receiver waits forever despite an empty queue. What might remain alive?
5. Why widen a partial sum before accumulation?

<details><summary>Answer key and reasoning</summary>

1. Every sender is dropped. Disconnected channels end iteration after queued messages drain.
2. Its lifetime is bounded by the scope. The scope joins its threads before borrowed inputs can disappear.
3. The addition may overflow. Use quotient and remainder to avoid the overflowing addition.
4. A sender clone prevents disconnection even when no messages are queued.
5. A narrow accumulator can overflow before its result is converted.

</details>
