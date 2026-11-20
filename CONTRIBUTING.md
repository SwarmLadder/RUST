# Contributing to SwarmLadder Rust

Thank you for improving the complete local-first Rust curriculum.

## Course structure and backwards compatibility

- Preserve the existing **36 workshops**, **156 original exercises**, **nine projects**, **eight linked study-log lessons** and their published test contracts.
- Mastery tasks use the public `swarmladder_rust::mastery` module, with matching function signatures in `mastery/starter.rs` and `mastery/reference.rs`, plus public shared error types in `mastery/types.rs`.
- Keep exercise starters intentionally incomplete using `todo!()`. Completed reference implementations must stay behind the `reference` Cargo feature.
- When adding or changing a mastery function, update acceptance tests, invariant tests, the corresponding level guide, curriculum, assessment and the portfolio milestones.
- Add a normal successful example, an invalid input or failure case, and a meaningful extreme/boundary test for each function.
- Avoid external dependencies for these mastery exercises. They are designed to work offline on stable Rust 2021.

## Testing and documentation

Run from the Rust repository root:

```sh
python3 tools/validate_course.py
cargo check --all-targets
cargo test --features reference
cargo doc --features reference --no-deps
cargo run --example mastery_demo --features reference
```

The reference-mode tests must pass on pull requests. Learner-mode full tests will fail until all `todo!()` functions are implemented and should **not** be made green by replacing starters with reference answers.

## Safety and technical accuracy

- Prefer safe Rust and standard-library primitives; explain any complexity that is genuinely needed.
- Avoid default-panic behavior on malformed input when a task contract promises a typed error.
- Enforce resource limits before allocation in binary-input tasks.
- Ensure no partial mutation on failure in transactions or rollback exercises.
- Distinguish process-local atomic correctness from distributed/persistent guarantees.
- Never include real credentials, API keys, external customer data, production endpoints or destructive deployment actions in sample code.
- Do not claim unverified performance, security or cloud-deployment guarantees.

## Pull-request review checklist

1. Is the changed material accurate and reproducible on stable Rust?
2. Do starter/reference API signatures still match?
3. Do valid/invalid and extreme cases have tests?
4. Are all documentation links local and valid?
5. Does the change preserve existing level tests?
6. Do the CI checks pass with the reference feature enabled?

Use [the final assessment](ASSESSMENT.md) and [portfolio criteria](PORTFOLIO.md) to keep learner outcomes measurable.
