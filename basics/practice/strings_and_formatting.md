# Strings and formatting

## Preparing text for display

String owns a growable UTF-8 buffer. A &str borrows an existing region. Formatting builds owned output, while splitting a borrowed string can return slices into the original buffer.

**Worked trace.** Splitting "Ada: notes: draft" once keeps "notes: draft" together. Repeated splitting would lose the distinction between the separator and punctuation in the body.

## Try it

```rust
let msg = "Ada: notes: draft";
let (author, body) = msg.split_once(':').unwrap();
assert_eq!(author, "Ada");
assert_eq!(body.trim_start(), "notes: draft");
```

Try a colon at the beginning and another at the end. Decide when a result is empty and when it is absent.

## Exercises

Edit [the starter functions](starter/strings_and_formatting.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `badge`

Trim the name and append a space and # followed by an ID at least four digits wide.

`cargo test --test basics_practice badge -- --exact`

### `subject`

Split at the first colon, trim both sides, preserve empty fields.

`cargo test --test basics_practice subject -- --exact`

### `initials`

Take the first scalar of each whitespace-separated word without changing case.

`cargo test --test basics_practice initials -- --exact`

### `reverse_text`

Reverse Unicode scalar values.

`cargo test --test basics_practice reverse_text -- --exact`

### `underline`

Produce one hyphen per Unicode scalar in s.

`cargo test --test basics_practice underline -- --exact`

## Review questions

1. Why split a subject line only once?
2. Will reversing scalar values always preserve displayed glyphs?

<details><summary>Check your reasoning</summary>

1. Later colons belong to the message body.
2. No. Combining marks can move away from the base characters they modify.

</details>

[Compare worked solutions](solutions/strings_and_formatting.rs) · [Return to the level guide](../README.md)
