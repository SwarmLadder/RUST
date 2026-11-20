# 7. Turn data into predictable text

A learner reads the report in a terminal. Another program might compare it with yesterday's output. Give both callers a stable format by returning the report as a String. The executable will decide where to write it.

## Experiment with the newline

```rust
let mut text = String::new();
for score in [2, 4] {
    text.push_str(&format!("score = {score}\n"));
}
assert_eq!(text.lines().count(), 2);
assert!(text.ends_with('\n'));
```

Remove the newline and predict the result of `lines().count()`. Two calls to `push_str` can still produce a single line.

## Implement render

For each key and value in the borrowed BTreeMap, append the topic, a colon and a space, the number of minutes, and the literal suffix ` min\n`. A map containing Zoo with 3 minutes and Algebra with 0 minutes should produce:

```text
Algebra: 0 min
Zoo: 3 min
```

The final row also ends in a newline. The empty-map case returns exactly `"No sessions.\n"`. A topic with zero minutes still gets its own row.

Run `cargo test lesson7`. If the terminal output looks right but a test fails, print the string with the debug formatter `{:?}` to reveal the exact spaces and newline characters.

[Next — build the command-line report](08-cli.md)
