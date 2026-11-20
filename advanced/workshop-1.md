# Lifetimes and borrowed views

Index a document without allocating new strings. Make the relation between a returned view and its backing storage explicit.

When two input references have different roles, lifetime elision may no longer say where the result comes from. In `fn prefix<'a>(text: &'a str, n: usize) -> Option<&'a str>`, the output borrows text. Use `str::get` for a checked byte range, because UTF-8 characters can span several bytes.

## Worked example

Predict the result before reading the assertions.

```rust
let s = "AéZ";
assert_eq!(s.get(1..3), Some("é"));
assert_eq!(s.get(2..3), None);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `before`

Borrow the prefix before the first nonempty delimiter. Return None if absent or empty.

`cargo test --test advanced before -- --exact`

### `byte_slice`

Return a checked half-open byte slice. Reject non-boundaries and reversed ranges.

`cargo test --test advanced byte_slice -- --exact`

### `field`

Find the first matching key in semicolon-separated key=value pairs. Trim both sides and skip malformed pairs.

`cargo test --test advanced field -- --exact`

## Knowledge check

1. Does a lifetime annotation extend storage duration? Choose between No or Yes.
2. Is byte offset 1 a boundary in 'é'? Choose between Yes or No.
3. May the key in field be dropped while the returned value survives? Choose between Yes, if the source line survives or No, the result borrows both.
4. Can an explicit lifetime annotation extend the lifetime of a local String?
5. Why is a character count insufficient for passing a byte range to get?

<details><summary>Answer key and reasoning</summary>

1. No. It describes a borrowing relationship.
2. No. The UTF-8 encoding occupies two bytes.
3. Yes, if the source line survives. The signature ties the output to line alone.
4. No. The owner still drops when its scope ends.
5. A scalar may occupy several bytes, so the offset may fall inside a UTF-8 sequence.

</details>
