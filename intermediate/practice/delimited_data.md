# Delimited data

## Reading a small interchange format

A format contract must define separators, escaping and missing fields. Start with a deliberately small grammar. Report the first bad field before turning the parsed row into an application record.

**Worked trace.** A literal separator may appear in an escaped field. A trailing escape marker is incomplete input, even after earlier fields parsed correctly.

## Try it

```rust
let input = "left\t\tright\t";
assert_eq!(input.split('\t').collect::<Vec<_>>(), ["left", "", "right", ""]);
```

Try split_whitespace on the same input. Identify the columns the table would lose.

## Exercises

Edit [the starter functions](starter/delimited_data.rs). The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `cells`

Split on tabs preserving empty fields, including the final one.

`cargo test --test intermediate_practice cells -- --exact`

### `escape`

Prefix vertical bars and backslashes with a backslash.

`cargo test --test intermediate_practice escape -- --exact`

### `decode_row`

Split at unescaped vertical bars. Backslash may escape only bar or backslash. Reject dangling escapes.

`cargo test --test intermediate_practice decode_row -- --exact`

### `encode_row`

Join escaped fields with bars. Reject a row with no fields.

`cargo test --test intermediate_practice encode_row -- --exact`

### `widths`

Return column count of a tab-separated table. Empty input has zero columns. Report one-based first unequal line.

`cargo test --test intermediate_practice widths -- --exact`

## Review questions

1. Why distinguish a row with no fields from one empty field?
2. How should a decoder handle a trailing backslash?

<details><summary>Check your reasoning</summary>

1. One empty field encodes as the empty string, leaving no distinct encoding for zero fields.
2. Reject it because the required escaped character is absent.

</details>

[Compare worked solutions](solutions/delimited_data.rs) · [Return to the level guide](../README.md)
