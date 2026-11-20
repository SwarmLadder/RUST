# Versioned state machines

## Applying an edit history

A revision number can reject updates based on stale state. Validate prerequisites before committing a new value. Replay should identify the first invalid operation and preserve enough information for the caller to diagnose it.

**Worked trace.** At revision 2 an edit expecting revision 1 is stale even if its value looks valid. Replaying the next edit against the old revision would hide that error.

## Try it

```rust
use swarmladder_rust::advanced_practice::Revision;
let initial = Revision { version: 3, value: 10 };
let proposed = Revision { version: 4, value: -1 };
let committed = if proposed.value >= 0 { proposed } else { initial };
assert_eq!(committed, initial);
```

A client reads version 3 twice. Trace what must happen when it submits two successive edits both expecting version 3.

## Exercises

Edit [the starter functions](starter/versioned_state_machines.rs). The [provided types](types.rs) define `Revision`. The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `apply`

Require the expected version and a representable next version. Return the updated state.

`cargo test --test advanced_practice apply -- --exact`

### `replay`

Apply edits sequentially or report the zero-based index of the first rejected edit.

`cargo test --test advanced_practice replay -- --exact`

### `commit`

Apply a batch atomically to the provided state. A rejected edit leaves both fields intact.

`cargo test --test advanced_practice commit -- --exact`

### `history`

Return the opening state followed by each successful state. Reject an invalid history with its edit index.

`cargo test --test advanced_practice history -- --exact`

### `rollback`

Find the state at a version in a history sorted by strictly increasing version. Use binary search.

`cargo test --test advanced_practice rollback -- --exact`

## Review questions

1. Should a stale edit with an identical value increment the version?
2. Why assign a replay result only after the entire batch succeeds?

<details><summary>Check your reasoning</summary>

1. No. The expected-version check still fails.
2. A failed batch must preserve the original version and value.

</details>

[Compare worked solutions](solutions/versioned_state_machines.rs) · [Return to the level guide](../README.md)
