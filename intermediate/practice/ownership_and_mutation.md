# Ownership and mutation

## Editing a playlist

Moving a String transfers its buffer. Borrowing lets the caller retain it. A function can take a mutable reference and still return owned data by replacing or removing the original value.

**Worked trace.** Taking the title from a mutable slot leaves an empty String there. A later assignment can populate the same slot without recovering the removed title.

## Try it

```rust
let mut queue = vec![String::from("mix"), String::from("play")];
let removed = queue.pop().unwrap();
assert_eq!(removed, "play");
assert_eq!(queue, ["mix"]);
```

Repeat with one element and then an empty vector. Which operation needs to check its precondition?

## Exercises

Edit [the starter functions](starter/ownership_and_mutation.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `take_title`

Move out the current title and leave an empty string.

`cargo test --test intermediate_practice take_title -- --exact`

### `rename`

Install the incoming title and return the old allocation.

`cargo test --test intermediate_practice rename -- --exact`

### `pop_pair`

Remove the final two items in original order. Leave a short list intact.

`cargo test --test intermediate_practice pop_pair -- --exact`

### `partition_titles`

Move nonblank titles to the first vector and whitespace-only ones to the second. Preserve text and order.

`cargo test --test intermediate_practice partition_titles -- --exact`

### `join_owned`

Append a borrowed suffix to the owned prefix and return it.

`cargo test --test intermediate_practice join_owned -- --exact`

## Review questions

1. Which operation moves a String out of a borrowed slot without cloning?
2. Why check playlist length before popping twice?

<details><summary>Check your reasoning</summary>

1. mem::take replaces it with its default value and returns the original.
2. A failed second pop would otherwise leave a short list changed.

</details>

[Compare worked solutions](solutions/ownership_and_mutation.rs) · [Return to the level guide](../README.md)
