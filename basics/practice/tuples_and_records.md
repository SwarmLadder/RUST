# Tuples and records

## Describing a delivery

Tuples fit short intermediate results. A struct gives the same data named fields and makes similarly typed quantities harder to mix up. Borrow a record when reading its fields, and borrow mutably when a caller needs the update.

**Worked trace.** A parcel carries pieces and grams per piece. Three pieces at 200 grams weigh 600 grams, while changing the number of pieces leaves unit weight intact.

## Try it

```rust
use swarmladder_rust::basics_practice::Parcel;
let parcel = Parcel { pieces: 3, unit_grams: 200 };
let borrowed = &parcel;
assert_eq!(u32::from(borrowed.pieces) * u32::from(borrowed.unit_grams), 600);
assert_eq!(parcel.pieces, 3);
```

Change both fields to 400. Predict the result of multiplying before widening.

## Exercises

Edit [the starter functions](starter/tuples_and_records.rs). The [provided types](types.rs) define `Parcel`. The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `parcel`

Construct a Parcel with the given pieces and unit_grams.

`cargo test --test basics_practice parcel -- --exact`

### `weight`

Compute total parcel weight in grams.

`cargo test --test basics_practice weight -- --exact`

### `heavier`

Decide whether the first parcel strictly outweighs the second.

`cargo test --test basics_practice heavier -- --exact`

### `unpack`

Consume a parcel and return quantity then unit weight.

`cargo test --test basics_practice unpack -- --exact`

### `combine`

Combine quantities only for equal unit weights and a representable total.

`cargo test --test basics_practice combine -- --exact`

## Review questions

1. Why borrow a Parcel to calculate its weight?
2. Can equal unit weights guarantee that a merge succeeds?

<details><summary>Check your reasoning</summary>

1. Reading two fields requires no ownership transfer. The caller can reuse the parcel.
2. No. Adding the two quantities can exceed u16::MAX.

</details>

[Compare worked solutions](solutions/tuples_and_records.rs) · [Return to the level guide](../README.md)
