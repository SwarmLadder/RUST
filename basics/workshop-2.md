# Text and Unicode

A label editor must handle international names. Inspect a label in three different ways: trim its edges, count Unicode scalar values and select its first scalar.

`"é".len()` counts two UTF-8 bytes. `"é".chars().count()` counts one scalar value. A displayed glyph can contain several scalars, so specify exactly what your count means.

## Worked example

Predict the result before reading the assertions.

```rust
let city = "  Łódź ".trim();
assert_eq!(city.chars().next(), Some('Ł'));
assert!(city.len() > city.chars().count());
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `label`

Trim surrounding whitespace and borrow the remaining text.

`cargo test --test basics label -- --exact`

### `scalar_count`

Count Unicode scalar values.

`cargo test --test basics scalar_count -- --exact`

### `initial`

Return the first scalar after trimming, or None.

`cargo test --test basics initial -- --exact`

## Knowledge check

1. What does str::len count? Choose between Bytes or User-visible characters.
2. What represents a missing initial? Choose between A space or None.
3. Does trim remove the space inside 'New York'? Choose between No or Yes.
4. What does trimming a whitespace-only label return?
5. Can two strings display as é but have different scalar counts?

<details><summary>Answer key and reasoning</summary>

1. Bytes. Rust strings store UTF-8 bytes.
2. None. An Option can express absence without reserving a character.
3. No. Only the edges are affected.
4. An empty borrowed string slice.
5. Yes. A base e followed by a combining accent has two scalars. Precomposed é has one.

</details>
