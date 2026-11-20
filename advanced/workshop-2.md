# Streaming I/O

Process a source through BufRead and a destination through Write. Use a Cursor in tests so no temporary files are needed.

`reader.lines()` yields `io::Result<String>` for each line. A failed read is different from reaching EOF. Use `?` to stop on I/O failure, and remember that the line terminator is removed.

## Worked example

Predict the result before reading the assertions.

```rust
use std::io::{BufRead, Cursor};
let mut input = Cursor::new(b"one\ntwo");
let mut line = String::new();
assert_eq!(input.read_line(&mut line).unwrap(), 4);
assert_eq!(line, "one\n");
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `line_count`

Count all physical lines, propagating invalid UTF-8 and other read errors.

`cargo test --test advanced line_count -- --exact`

### `write_rows`

Write every row followed by one newline and propagate write errors.

`cargo test --test advanced write_rows -- --exact`

### `first_line`

Find the first line containing needle, returning its one-based number. Stop reading once found.

`cargo test --test advanced first_line -- --exact`

## Knowledge check

1. What does EOF produce for an empty source? Choose between Zero lines or One empty line.
2. Which trait lets the same function write to a file or Vec<u8>? Choose between BufRead or Write.
3. Should first_line report an error after a successful match? Choose between No or Yes.
4. How do you distinguish EOF from an empty line using read_line?
5. Why can a writer fail after some rows were emitted?

<details><summary>Answer key and reasoning</summary>

1. Zero lines. No line was read.
2. Write. Both can implement the byte-output interface.
3. No. The contract stops at the first match without reading further.
4. EOF returns Ok(0). An empty terminated line consumes at least the newline.
5. I/O may fail on any write. Returning an error cannot reverse earlier writes.

</details>
