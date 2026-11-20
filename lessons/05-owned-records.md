# 5. Keep a record after its input goes away

In lesson 3, `parse_entry` returned a topic that borrowed from a line. Now imagine reading a log in a small scope and passing the parsed sessions to another function later. Which value will own the letters in “Ownership” once the input string is dropped?

## Try the ownership boundary

```rust
let label = {
    let input = String::from("  Ownership  ");
    input.trim().to_owned()
};
assert_eq!(label, "Ownership");
```

Predict what happens if you remove `.to_owned()`. The compiler should point to the end of the inner block. The borrowed slice would still need the input there. Copying the trimmed text into a new String gives the label its own storage. Here you pay for that copy once per record.

## Implement sessions

Open `src/exercises.rs`. The supplied `Session` struct has a `String` topic and a `u32` duration. Build a `Vec<Session>` in input order. Use your existing `parse_entry` for validation.

| Physical line | Input | Action |
| --- | --- | --- |
| 1 | `  Traits ,20` | Append a session with topic Traits and duration 20 |
| 2 | blank | Skip it |
| 3 | `Rust,0` | Append a session with duration zero |
| 4 | `Rust,ten` | Return the error for line 4 |

Enumerate before skipping blank lines. On the first error, return the one-based line number with the original `EntryError`. A completely blank file produces an empty vector.

Run `cargo test lesson5`. If an assertion reports line 3 instead of 4, check where your code counts lines. If an ownership error mentions a temporary value, check the type of the topic you store.

[Next — group minutes by topic](06-topic-totals.md)
