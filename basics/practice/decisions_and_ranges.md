# Decisions and ranges

## Checking admission and fares

Order guards so a special case wins before a broader range. Inclusive ranges make both endpoints visible. When ranges overlap on paper, resolve the ambiguity before translating the rules into code.

**Worked trace.** A child aged 5 travels free. At 6 the child rate begins. Test both ages to establish which range owns the boundary.

## Try it

```rust
let age = 17;
let student_age = (6..=17).contains(&age);
assert!(student_age);
assert!(!(6..=17).contains(&(age + 1)));
```

Move the student upper bound down by one. Which passenger is affected?

## Exercises

Edit [the starter functions](starter/decisions_and_ranges.rs). The [acceptance tests](../../tests/basics_practice.rs) show example inputs and expected results.

### `fare`

Return fare 0 through age 5, 2 for ages 6–17, 3 from age 65, otherwise 5.

`cargo test --test basics_practice fare -- --exact`

### `leap`

Apply Gregorian leap-year rules.

`cargo test --test basics_practice leap -- --exact`

### `band`

Assign A for 90–100, B for 80–89, C below 80. Reject scores over 100.

`cargo test --test basics_practice band -- --exact`

### `inside`

Include both endpoints even when supplied backwards.

`cargo test --test basics_practice inside -- --exact`

### `quadrant`

Number quadrants counterclockwise from upper right. Return None on either axis.

`cargo test --test basics_practice quadrant -- --exact`

## Review questions

1. Which leap-year test distinguishes 1900 from 2000?
2. Where do points on the axes belong?

<details><summary>Check your reasoning</summary>

1. Both are century years, but only 2000 is divisible by 400.
2. Outside the four numbered quadrants, so the function returns None.

</details>

[Compare worked solutions](solutions/decisions_and_ranges.rs) · [Return to the level guide](../README.md)
