# Graph traversal

## Finding a dependency path

An adjacency list stores outgoing neighbors for each vertex. A visited set prevents cycles from expanding indefinitely. Breadth-first traversal discovers the fewest-edge route when edges have equal cost.

**Worked trace.** In a diamond with edges 0→1, 0→2, 1→3 and 2→3, preserving neighbor order makes the path through 1 the first shortest path.

## Try it

```rust
use std::collections::VecDeque;
let g = [vec![1, 2], vec![3], vec![3], vec![]];
let mut frontier = VecDeque::from([0]);
let first = frontier.pop_front().unwrap();
frontier.extend(g[first].iter().copied());
assert_eq!(frontier, [1, 2]);
```

Continue the trace through vertex 2. Show exactly when vertex 3 becomes visited.

## Exercises

Edit [the starter functions](starter/graph_traversal.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `valid_graph`

Check that every neighbor is a valid vertex index.

`cargo test --test advanced_practice valid_graph -- --exact`

### `transpose`

Reverse every edge in source traversal order. Reject invalid graphs. Preserve parallel edges.

`cargo test --test advanced_practice transpose -- --exact`

### `distances`

Return fewest-edge distances. Reject invalid graph or start. Unreachable vertices have None.

`cargo test --test advanced_practice distances -- --exact`

### `path`

Return first shortest path in neighbor order. Reject invalid or unreachable requests.

`cargo test --test advanced_practice path -- --exact`

### `topological`

Return a topological order, choosing smallest currently ready vertex. Reject invalid or cyclic graphs.

`cargo test --test advanced_practice topological -- --exact`

## Review questions

1. Why mark a vertex visited when enqueuing it?
2. What does a partially completed topological ordering indicate on a valid finite graph?

<details><summary>Check your reasoning</summary>

1. It prevents duplicate discovery and preserves the first shortest predecessor.
2. The remaining vertices are blocked by at least one directed cycle.

</details>

[Compare worked solutions](solutions/graph_traversal.rs) · [Return to the level guide](../README.md)
