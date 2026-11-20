# Zero-copy parsing

## Reading borrowed protocol fields

A parser can return both a token and the remaining input as borrowed slices. This keeps the original buffer responsible for storage. Use byte offsets only after establishing a UTF-8 boundary.

**Worked trace.** Parsing "ab:12" yields the borrowed key "ab" and the remainder "12". A borrowed empty field still points into the input.

## Try it

```rust
let bytes = "Aé: data";
let sep = bytes.find(':').unwrap();
assert_eq!(sep, 3);
assert_eq!(&bytes[..sep], "Aé");
```

Trace the same example using a scalar index for the colon. Which byte would that index point into?

## Exercises

Edit [the starter functions](starter/zero_copy_parsing.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `token`

Skip leading whitespace, take one word, leave the suffix starting at its following whitespace.

`cargo test --test advanced_practice token -- --exact`

### `quoted`

Read a double-quoted prefix without escapes. Return its contents and unconsumed suffix.

`cargo test --test advanced_practice quoted -- --exact`

### `header`

Split at first colon. Require a nonempty ASCII alphabetic key. Trim only the value.

`cargo test --test advanced_practice header -- --exact`

### `take_chars`

Split after exactly n Unicode scalars or fail if too short.

`cargo test --test advanced_practice take_chars -- --exact`

### `find_header`

Return the first valid matching header value using ASCII case-insensitive key comparison. Skip invalid lines.

`cargo test --test advanced_practice find_header -- --exact`

## Review questions

1. Can a borrowed header outlive the response buffer?
2. Why skip leading whitespace before searching for a token boundary?

<details><summary>Check your reasoning</summary>

1. No. The header slice borrows that buffer.
2. Otherwise the first whitespace could produce an empty token without advancing the parser.

</details>

[Compare worked solutions](solutions/zero_copy_parsing.rs) · [Return to the level guide](../README.md)
