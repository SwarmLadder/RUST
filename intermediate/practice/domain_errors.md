# Domain errors

## Reading configuration settings

An error enum preserves distinctions callers can act on. Convert parse errors at the boundary and propagate them with ?. Choose an order of validation when input violates more than one rule.

**Worked trace.** For a missing separator, parsing the number is premature. After a split succeeds, checking the key first gives the caller a stable diagnostic.

## Try it

```rust
use swarmladder_rust::intermediate_practice::ConfigError;
let parsed = "oops".parse::<u16>().map_err(|_| ConfigError::InvalidValue);
assert_eq!(parsed, Err(ConfigError::InvalidValue));
```

Try a syntactically valid number larger than u16::MAX. Does the domain need a separate error variant?

## Exercises

Edit [the starter functions](starter/domain_errors.rs). The [provided types](types.rs) define `ConfigError`. The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `key`

Trim and reject empty setting keys with EmptyKey.

`cargo test --test intermediate_practice key -- --exact`

### `quantity`

Trim and parse a u16 or return InvalidValue. Zero is valid.

`cargo test --test intermediate_practice quantity -- --exact`

### `setting`

Require an equals sign, then a valid key, then a quantity, in that order.

`cargo test --test intermediate_practice setting -- --exact`

### `settings`

Read every line as a setting. Attach one-based line numbers to errors.

`cargo test --test intermediate_practice settings -- --exact`

### `all_errors`

Collect every erroneous line and its error in input order.

`cargo test --test intermediate_practice all_errors -- --exact`

## Review questions

1. Why preserve different config errors as enum variants?
2. When is collecting every error preferable?

<details><summary>Check your reasoning</summary>

1. Callers can match on the cause without parsing error text.
2. When validating an entire configuration for a person who can correct several lines at once.

</details>

[Compare worked solutions](solutions/domain_errors.rs) · [Return to the level guide](../README.md)
