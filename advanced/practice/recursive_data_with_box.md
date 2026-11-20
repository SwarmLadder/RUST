# Recursive data with Box

## Interpreting an expression tree

A recursive enum needs indirection to give it a finite size. Box owns a child node. Folding the tree bottom-up lets child errors reach the root before a parent operation is applied.

**Worked trace.** In (3 + 2) × 4, the root is multiplication. A postorder walk emits its two operands before the multiplication operator.

## Try it

```rust
use swarmladder_rust::advanced_practice::Expr;
let root = Expr::product(Expr::lit(6), Expr::lit(7));
if let Expr::Mul(left, right) = root {
    assert_eq!((*left, *right), (Expr::lit(6), Expr::lit(7)));
} else { panic!("expected a product"); }
```

Sketch the tree for 6 × (7 + 1). Which nodes must be evaluated before the root?

## Exercises

Edit [the starter functions](starter/recursive_data_with_box.rs). The [provided types](types.rs) define `Expr`. The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `nodes`

Count all literals and operators.

`cargo test --test advanced_practice nodes -- --exact`

### `depth`

Return tree height counting the root as one.

`cargo test --test advanced_practice depth -- --exact`

### `compute`

Evaluate using checked arithmetic at every internal node.

`cargo test --test advanced_practice compute -- --exact`

### `postfix`

Emit literals and operators in postfix order, separated by single spaces.

`cargo test --test advanced_practice postfix -- --exact`

### `map_literals`

Consume the tree and transform every literal, preserving operators and shape.

`cargo test --test advanced_practice map_literals -- --exact`

## Review questions

1. Why can an Expr contain Box<Expr> but not an inline Expr child?
2. When should a parent propagate an overflowing subexpression?

<details><summary>Check your reasoning</summary>

1. A box has fixed pointer size. An inline recursive child would make the enum infinitely large.
2. Before applying its own operator, even if combining mathematical values could fit afterward.

</details>

[Compare worked solutions](solutions/recursive_data_with_box.rs) · [Return to the level guide](../README.md)
