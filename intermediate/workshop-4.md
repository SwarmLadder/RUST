# Iterator pipelines

Combine operations over sequences without collecting every intermediate stage. Make empty input and overflow explicit.

`filter_map` keeps successful extractions, whereas collecting a sequence of Results can stop on the first failure. Pick the behavior your caller requested before choosing either technique.

## Worked example

Predict the result before reading the assertions.

```rust
let ids: Vec<u16> = ["2", "-1", "7"].into_iter().filter_map(|s| s.parse().ok()).collect();
assert_eq!(ids, [2, 7]);
```

## Practice

Edit [exercises.rs](exercises.rs). Run each test from the repository root.

### `valid_ids`

Keep successfully parsed, trimmed u32 values. Preserve duplicates.

`cargo test --test intermediate valid_ids -- --exact`

### `running`

Return each cumulative sum, widening before addition.

`cargo test --test intermediate running -- --exact`

### `dot`

Compute a dot product for equal lengths. Return None on mismatch.

`cargo test --test intermediate dot -- --exact`

## Knowledge check

1. Why compare lengths before zip? Choose between zip stops at the shorter input or zip panics on empty input.
2. How many outputs does running produce for three inputs? Choose between 1 or 3.
3. Which operation can skip parse failures? Choose between filter_map with Result::ok or map alone.
4. Why check lengths before computing a dot product with zip?
5. When should parsing stop at the first invalid element?

<details><summary>Answer key and reasoning</summary>

1. zip stops at the shorter input. Otherwise a missing component would be silently ignored.
2. 3. It emits each intermediate total.
3. filter_map with Result::ok. None values disappear when filter_map is consumed.
4. Zip stops at the shorter iterator and silently discards the rest.
5. When the output promises every input was valid or the caller needs a diagnostic.

</details>
