# Stacks and queues

## Scheduling work

A stack removes the most recently appended item. A queue processes the oldest one. VecDeque supports both ends, which makes it suitable when unfinished jobs return to the back.

**Worked trace.** Round-robin service of jobs A with two turns and B with one produces A, B, A. A zero-turn job never enters the queue.

## Try it

```rust
use std::collections::VecDeque;
let mut jobs = VecDeque::from(["A", "B"]);
let first = jobs.pop_front().unwrap();
jobs.push_back(first);
assert_eq!(jobs.into_iter().collect::<Vec<_>>(), ["B", "A"]);
```

Replace push_back with push_front. Explain which job could be kept waiting.

## Exercises

Edit [the starter functions](starter/stacks_and_queues.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `balanced`

Match (), [] and {} with proper nesting. Ignore other characters.

`cargo test --test intermediate_practice balanced -- --exact`

### `undo`

Some pushes a value. None undoes the last surviving push. Empty undo is harmless.

`cargo test --test intermediate_practice undo -- --exact`

### `round_robin`

Emit one job name per turn. Requeue unfinished jobs and skip zero-turn jobs.

`cargo test --test intermediate_practice round_robin -- --exact`

### `recent`

Keep at most the final capacity events, oldest first.

`cargo test --test intermediate_practice recent -- --exact`

### `evaluate`

Evaluate postfix i32 tokens with checked + and -. Require exactly one final value.

`cargo test --test intermediate_practice evaluate -- --exact`

## Review questions

1. Why does a count of open brackets fail on ([)]?
2. Which number is subtracted in a postfix subtraction?

<details><summary>Check your reasoning</summary>

1. It loses the nesting order and cannot pair each closer with the latest opener.
2. The top value is the right operand and is subtracted from the next value.

</details>

[Compare worked solutions](solutions/stacks_and_queues.rs) · [Return to the level guide](../README.md)
