# Borrow a title

A learner enters a topic with spaces around it. The caller owns the input and wants to keep using it after validation. Your function can borrow a slice of the original text.

```rust
let input = String::from("  loops  ");
let cleaned: &str = input.trim();
assert_eq!(cleaned, "loops");
assert_eq!(input.len(), 9);
```

Implement `title` in `src/exercises.rs`. Return `Some(trimmed)` for a nonempty title and `None` when the input contains only whitespace. Keep internal spaces and Unicode characters intact. The returned slice must remain valid for as long as its input borrow permits. Returning a reference to a newly created local String would leave the caller pointing at dropped storage.

Predict the result for an empty string and for `"  café notes "`. Check your work with `cargo test lesson2`. For an extra experiment, keep a slice of a String, try to clear the String, then print the slice. Read the compiler's explanation of the conflicting borrows.

Continue to [Parse a session](03-entries.md).
