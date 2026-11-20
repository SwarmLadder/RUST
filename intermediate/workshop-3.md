# Generics and predicates

Separate an algorithm from its element type. Give it just the operations it needs through trait bounds and closure parameters.

A function comparing two elements can require `T: Ord`. A function that filters elements can instead accept a predicate, so the caller owns the policy. Borrowing a result often removes the need for a Clone bound.

## Worked example

Predict the result before reading the assertions.

```rust
let mut calls = 0;
let mut accepts = |x: i32| { calls += 1; x > 1 };
assert!(accepts(2));
assert_eq!(calls, 1);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `larger`

Borrow the larger value, choosing the first on a tie.

`cargo test --test intermediate larger -- --exact`

### `tally`

Count matches using a stateful predicate.

`cargo test --test intermediate tally -- --exact`

### `last_match`

Return the last match. Visit items from the start, once each.

`cargo test --test intermediate last_match -- --exact`

## Knowledge check

1. Why can larger work without Clone? Choose between It returns an existing reference or Ord also supplies Clone.
2. Which closure trait permits changing a captured counter? Choose between Fn only or FnMut.
3. Can last_match reverse the iteration here? Choose between No or Yes.
4. Why can returning &T avoid a Clone bound?
5. What does a mutable captured counter require of the closure caller?

<details><summary>Answer key and reasoning</summary>

1. It returns an existing reference. No owned value is copied out.
2. FnMut. Mutating captured state may require mutable access to the closure.
3. No. A stateful predicate observes visit order.
4. It shares access to the caller’s existing element.
5. The caller must support FnMut and invoke the closure through a mutable binding.

</details>
