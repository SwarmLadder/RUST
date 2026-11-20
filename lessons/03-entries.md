# Parse a session

Your input file has one entry per line, such as `Ownership,25`. A useful parser tells the caller which part failed. This function returns `Result<(&str, u32), EntryError>`.

Try the building blocks with a different delimiter:

```rust
let pair = "tries=3".split_once('=');
assert_eq!(pair, Some(("tries", "3")));
assert_eq!("3".parse::<u32>(), Ok(3));
assert!("three".parse::<u32>().is_err());
```

Implement `parse_entry` in `src/exercises.rs`. Apply the checks in order:

1. Split at the first comma. A missing comma gives `MissingSeparator`.
2. Reuse your title function. A blank title gives `EmptyTitle`.
3. Trim the minutes and parse them as a `u32`. Return `InvalidMinutes` for malformed or out-of-range values. Zero is valid.

Use `ok_or` to turn an Option into a Result. Use `map_err` to translate the numeric parser's error into the course enum. The `?` operator extracts a successful value or returns the error to the caller.

Run `cargo test lesson3`. Which error should `" ,bad"` produce? Why does `"Rust,20,30"` fail even though it contains a valid title?

Continue to [Build the study-log summary](04-summary.md).
