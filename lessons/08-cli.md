# 8. Project — a study report from standard input

A classmate keeps a file of study sessions. They want a command that they can run after each update. This is the sample file in `fixtures/week.txt`:

```text
Ownership,25
 Traits ,15

Ownership,10
```

Before coding, write down the expected order of the topics and the total for each. Then change the final duration to a word. Which line should the error identify?

## Connect the stages

Implement `report` in `src/exercises.rs`. Parse the entire input with `sessions`. Keep the resulting vector alive while you call `totals`, then pass the map to `render`. On success, the returned String owns its text, so both intermediate collections can be dropped.

On failure, convert the line number and typed error to the following diagnostic. Return the message without a trailing newline. The supplied executable adds one when writing to standard error.

| Parser error | Message for physical line 4 |
| --- | --- |
| MissingSeparator | `line 4: expected a comma` |
| EmptyTitle | `line 4: topic is blank` |
| InvalidMinutes | `line 4: minutes must be a nonnegative u32` |

Use an exhaustive `match` to translate the enum. Adding another enum variant in the future will make the compiler identify the translation that needs updating.

## Run the command

The supplied `src/bin/study-report.rs` reads standard input until EOF. In a POSIX shell, redirect the sample file into the program:

```sh
cargo run --quiet --bin study-report < fixtures/week.txt
```

Expected output:

```text
Ownership: 35 min
Traits: 15 min
```

Now run `printf 'Ownership,25\n\nRust,-1\n' | cargo run --quiet --bin study-report`. It should write `line 3: minutes must be a nonnegative u32` to standard error and exit unsuccessfully. Standard output should stay empty. Parsing all records before rendering prevents a misleading partial report.

Run `cargo test lesson8`, then `cargo test`. The process tests send input to the actual executable and check output, diagnostics and exit status separately. To explore further, write a test for a whitespace-only file going through the executable.
