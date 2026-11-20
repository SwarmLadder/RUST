# Project — summarize a study log

Turn several lines of learner activity into one report. Your parser already validates individual entries. The report adds two things the parser cannot know: the running total and the position of a bad entry.

```rust
let text = "first\n\nthird";
let numbered: Vec<_> = text.lines().enumerate().collect();
assert_eq!(numbered[2], (2, "third"));
```

Implement `summarize` in `src/exercises.rs`. Initialize a Summary with zero sessions and minutes. Enumerate physical lines before skipping blank ones so diagnostics keep their original line numbers. For each nonblank line, call `parse_entry`. If it fails, immediately return the one-based line number and error. Otherwise increment the count and add the minutes as a `u64` so two large valid entries can be summed.

Run `cargo test lesson4`, then `cargo test` for the whole course. Finally run `cargo run --example study_log`. Its two entries should print `2 sessions, 65 minutes`. Change the second duration to `forty` and verify the example identifies line 3.

Extension: Add your own regression test for a valid zero-minute session followed by a malformed line.
