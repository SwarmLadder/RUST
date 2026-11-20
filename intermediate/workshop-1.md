# Validation with Result

Write a configuration reader that distinguishes missing input from invalid input. Choose Result when callers need a reason for failure.

`raw.trim().parse::<u16>()` produces a Result. `map_err` translates parser-specific errors, while `?` propagates the translated error from the enclosing function. Validate in the order promised to callers.

## Worked example

Predict the result before reading the assertions.

```rust
let value = " 19 ".trim().parse::<u16>();
assert_eq!(value, Ok(19));
assert!("65536".parse::<u16>().is_err());
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `port`

Trim and parse a port. Return 'invalid' for a non-u16 and 'zero' for port 0.

`cargo test --test intermediate port -- --exact`

### `required`

Borrow the trimmed value or return 'empty'.

`cargo test --test intermediate required -- --exact`

### `numbers`

Parse every trimmed line. Return the first invalid physical line number, starting at one.

`cargo test --test intermediate numbers -- --exact`

## Knowledge check

1. What does ? do on Err? Choose between Returns an error from this function or Returns a zero value.
2. Where should zero be rejected in port? Choose between After converting to a string again or After parsing.
3. Which failure should numbers report? Choose between Earliest invalid line or Count of all failures.
4. Why annotate an error before collecting parsed lines?
5. What is lost if you call .ok() before reporting a parse failure?

<details><summary>Answer key and reasoning</summary>

1. Returns an error from this function. It short-circuits the success path.
2. After parsing. The numeric value is available after parsing.
3. Earliest invalid line. The return type and contract describe a single line number.
4. At the point of parsing you still have the source line number.
5. The error value and its cause are discarded.

</details>
