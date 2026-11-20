# Working with generic streams

## Copying bounded input

Read may return fewer bytes than requested. A fixed-size header needs read_exact, while a bounded copy should stop at EOF. Write can also make partial progress. write_all retries until the complete slice is accepted or an error occurs.

**Worked trace.** A two-byte destination limit should leave the next source byte unread. Reading the whole source into memory first would violate that contract for a shared stream.

## Try it

```rust
use std::io::{Cursor, Read};
let mut input = Cursor::new(b"abcd");
let mut prefix = [0u8; 2];
input.read_exact(&mut prefix).unwrap();
assert_eq!((&prefix, input.position()), (b"ab", 2));
```

Use a one-byte source instead. After read_exact fails, may you assume the cursor stayed at zero?

## Exercises

Edit [the starter functions](starter/working_with_generic_streams.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `read_number`

Read exactly four bytes as a little-endian u32. Propagate a short read.

`cargo test --test advanced_practice read_number -- --exact`

### `bounded`

Copy at most n bytes, returning the count and leaving later bytes unread.

`cargo test --test advanced_practice bounded -- --exact`

### `write_record`

Write a two-byte big-endian length and complete payload. Reject oversized input with InvalidInput before writing.

`cargo test --test advanced_practice write_record -- --exact`

### `last_nonblank`

Return the last nonblank line with its original surrounding spaces, excluding the line terminator.

`cargo test --test advanced_practice last_nonblank -- --exact`

### `peek`

Inspect the next byte without consuming it. Empty input returns None.

`cargo test --test advanced_practice peek -- --exact`

## Review questions

1. Why use write_all for a payload?
2. How can a test prove peek leaves the source intact?

<details><summary>Check your reasoning</summary>

1. A successful write can accept fewer bytes than requested.
2. Read or inspect the following byte after peeking and verify it is still the same byte.

</details>

[Compare worked solutions](solutions/working_with_generic_streams.rs) · [Return to the level guide](../README.md)
