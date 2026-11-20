# Rust Glossary — Concepts Used in This Course

| Term | Working definition |
|---|---|
| Ownership | Each Rust value has an owner controlling when the value is dropped |
| Move | Ownership transfers, typically making the prior binding unusable for non-`Copy` values |
| Borrow | A reference to data owned elsewhere; lifetimes constrain how long it is valid |
| `&T` | Shared, immutable reference to `T` |
| `&mut T` | Exclusive mutable reference to `T` |
| Lifetime | Relationship governing validity of references; does not extend an allocation's life |
| Slice | Borrowed view into contiguous elements, e.g., `&[u8]` |
| `&str` | Borrowed UTF-8 text slice, indexed by valid byte boundaries |
| Unicode scalar | Value represented by Rust's `char`; not necessarily a displayed grapheme |
| `String` | Owned, growable UTF-8 string |
| `Option<T>` | Value may be present (`Some`) or absent (`None`) |
| `Result<T,E>` | Operation returns a success value or a typed error |
| `?` operator | Propagates a compatible error or absence from the current function |
| `BTreeMap` | Ordered key/value map; iteration proceeds in key order |
| `HashMap` | Hash-table map whose iteration order should not be assumed to be sorted |
| `checked_add` | Returns `None` instead of overflowing |
| `saturating_add` | Caps a result at the integer bound instead of overflowing |
| `TryFrom` | Fallible conversion for values that might not fit the target type |
| `Read` trait | Standard-library abstraction over byte sources |
| `read_exact` | Read a fixed number of bytes or return an error |
| Framing | Protocol for splitting a byte stream into records or messages |
| Backpressure | Mechanism preventing producers from outpacing consumers or bounded resources |
| Scoped thread | Thread that can safely borrow nonstatic data because its scope joins it |
| `Arc<T>` | Thread-safe reference-counted ownership (not automatic mutation safety) |
| `Mutex<T>` | Lock protecting exclusive access to shared mutable data |
| `AtomicUsize` | Atomic integer supporting operations such as load, store and CAS |
| Compare-and-swap (CAS) | Conditional atomic update applied only if the observed value is unchanged |
| Memory ordering | Rules governing visibility/reordering of atomic operations across threads |
| Transactional update | All-or-nothing change: either commit an entire valid batch or change nothing |
| Determinism | Same valid input produces the same output regardless of irrelevant execution order |
| `todo!()` | Intentional panic placeholder used to mark unfinished exercise code |
| Cargo feature | Opt-in compilation configuration, e.g., the course's `reference` solution feature |
| Reference mode | Course solution implementations used for curriculum validation; **not** student completion |

For guided examples see [the Rust mastery track](mastery/README.md). For API and safety choices see [the cheatsheet](CHEATSHEET.md).
