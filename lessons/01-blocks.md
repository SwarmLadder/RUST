# Divide a study session into blocks

You have 64 minutes before your next meeting. How many complete 25-minute focus blocks fit? How many minutes remain?

Rust can return two values as a tuple. Integer division discards the fractional part. The remainder operator gives you what the division left over. Try a smaller example first:

```rust
let seconds: u32 = 125;
let (minutes, leftover) = (seconds / 60, seconds % 60);
assert_eq!((minutes, leftover), (2, 5));
```

Implement `blocks` in `src/exercises.rs`. Rename the starter parameter to `minutes`, then return a tuple whose first element is the number of blocks. A final expression without a semicolon becomes the return value.

Predict the output for 24, 25 and 26 minutes. Check your implementation:

```sh
cargo test lesson1
```

If the compiler reports that it found `()` instead of a tuple, inspect the final semicolon. Continue to [Borrow a title](02-titles.md).
