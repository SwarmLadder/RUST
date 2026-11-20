# Trait-based interfaces

## Rendering heterogeneous data

A trait describes behavior independently of representation. A generic parameter selects one concrete type per call. A trait object lets values of different concrete types share a collection when the required methods are object-safe.

**Worked trace.** A string and a numeric ID can both provide a label. A slice of borrowed trait objects can request each label without knowing how it was produced.

## Try it

```rust
use swarmladder_rust::intermediate_practice::Label;
let id = 7u32;
let name = String::from("Delta");
let inputs: [&dyn Label; 2] = [&id, &name];
assert_eq!(inputs.map(|v| v.label()), ["7", "Delta"]);
```

Implement Label for a new record in your own example. Which code can you reuse unchanged?

## Exercises

Edit [the starter functions](starter/trait_based_interfaces.rs). The [provided types](types.rs) define `Label`. The [acceptance tests](../../tests/intermediate_practice.rs) show example inputs and expected results.

### `render`

Use the supplied Label trait to render one label surrounded by brackets.

`cargo test --test intermediate_practice render -- --exact`

### `labels`

Render each raw label in order, allowing mixed input types.

`cargo test --test intermediate_practice labels -- --exact`

### `render_all`

Join bracketed labels with a comma and no spaces.

`cargo test --test intermediate_practice render_all -- --exact`

### `same_label`

Compare labels across different concrete types.

`cargo test --test intermediate_practice same_label -- --exact`

### `longest_label`

Choose the first longest label by Unicode scalar count.

`cargo test --test intermediate_practice longest_label -- --exact`

## Review questions

1. Can a homogeneous slice &[T] directly store a String and u32 together?
2. Why keep the first longest label when lengths tie?

<details><summary>Check your reasoning</summary>

1. No. One concrete T applies to every element. A slice of references to dyn Label can refer to both.
2. The contract specifies source-order tie breaking, so replace the winner only for a strictly longer label.

</details>

[Compare worked solutions](solutions/trait_based_interfaces.rs) · [Return to the level guide](../README.md)
