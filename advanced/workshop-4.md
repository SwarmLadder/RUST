# Atomic state transitions

Model a batch update as a transaction: a failing operation must leave the original state intact. Test the state after failure as carefully as the error itself.

A simple approach is to calculate a candidate state, validate it, and write it back only after success. Checked arithmetic distinguishes an overflow from a valid sum. Real concurrent transactions need additional coordination; this workshop owns an exclusive mutable reference.

## Worked example

Predict the result before reading the assertions.

```rust
let original = 8u32;
let candidate = original.checked_sub(3).and_then(|v| v.checked_sub(6));
assert_eq!(candidate, None);
assert_eq!(original, 8);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `apply_delta`

Return the adjusted balance if representable, including i32::MIN.

`cargo test --test advanced apply_delta -- --exact`

### `batch`

Apply all changes atomically. Return the zero-based index of the first invalid delta.

`cargo test --test advanced batch -- --exact`

### `transfer`

Reject unknown or identical accounts, insufficient funds, and destination overflow. Commit both balances together.

`cargo test --test advanced transfer -- --exact`

## Knowledge check

1. Which balance should be visible after a rejected batch? Choose between The original balance or The last valid intermediate balance.
2. Does a saturating operation detect overflow for its caller? Choose between Yes or No.
3. When should a transfer debit its source? Choose between After both new balances validate or Before checking the destination.
4. Which two assertions belong in a rollback test?
5. Can checking only the sender balance guarantee a valid transfer?

<details><summary>Answer key and reasoning</summary>

1. The original balance. The batch is all-or-nothing.
2. No. It returns a clamped value; checked arithmetic returns None.
3. After both new balances validate. Otherwise a destination overflow could lose funds.
4. Check the error position and verify the entire state equals its opening snapshot.
5. No. The recipient may overflow, and both indices must also be valid.

</details>
