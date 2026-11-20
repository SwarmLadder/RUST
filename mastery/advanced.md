# Advanced Mastery — Bounded I/O, Scoped Threads and Atomic State

**Four graded tasks:** A01–A04 · **Expected effort:** 7–10 hours · **Prerequisites:** [Advanced workshops](../advanced/README.md), `Read`, `io::Error`, scoped threads, ownership, `AtomicUsize`, `Result` and checked arithmetic.

## Why this matters

Rust prevents many memory-safety bugs, but you still need to make correct engineering decisions about partial reads, memory budgets, failure rollback and concurrent state transitions. These exercises practice *operational correctness* as well as language constructs, using only the standard library.

## Worked example — checked arithmetic makes failure explicit

```rust
fn seat_total(current: usize, request: usize, limit: usize) -> Option<usize> {
    let next = current.checked_add(request)?;
    (next <= limit).then_some(next)
}
assert_eq!(seat_total(8, 2, 10), Some(10));
assert_eq!(seat_total(8, 3, 10), None);
assert_eq!(seat_total(usize::MAX, 1, usize::MAX), None);
```

The function works on one thread. A04 adds the additional challenge: more than one thread may read the *same* old value concurrently, so a simple load-then-store would lose updates. A compare-exchange loop must re-evaluate the state after contention.

## A01 — Parse a resource-bounded framed byte stream

**Signature:** `read_frames<R: Read>(reader: R, max_payload: usize, max_frames: usize) -> io::Result<Vec<Vec<u8>>>`

**Protocol:** Each frame starts with a **two-byte, unsigned big-endian** payload length followed immediately by exactly that many bytes. The length 0 is legal. Clean EOF *between* complete frames returns all frames. A dangling one-byte header or incomplete payload returns `UnexpectedEof`. Reject any payload length larger than `max_payload` and any additional frame after `max_frames` with `InvalidData`. Enforce frame-size limits **before allocating** its payload buffer. Do not assume one `Read::read` call fills the requested buffer.

**Trace:** bytes `00 00 00 02 61 62` → empty frame then payload `b"ab"`. Bytes `00 03 61 62` → length claims 3 bytes, but only 2 follow → `UnexpectedEof`.

**APIs:** `Read::read`, `Read::read_exact`, `u16::from_be_bytes`, `io::ErrorKind` and `Vec::push`.

**Failure clinic:** Reading the entire stream with `read_to_end` first defeats incremental resource controls. Allocating from an untrusted length before checking max payload invites resource exhaustion. Treat a partial header as a truncated frame, not as a clean EOF.

**Scope note:** This exercise limits *individual frame size* and the *count of frames*, not a global byte budget or CPU time. It is an educational parser, **not** a hardened network protocol implementation.

**Test:** `cargo test --test mastery a01_frames`.

## A02 — Aggregate byte histograms using scoped workers

**Signature:** `parallel_histogram(bytes: &[u8], workers: usize) -> Option<[usize; 256]>`

**Contract:** Return `None` when workers is zero. Otherwise, split borrowed input into contiguous chunks, let workers count independently, then combine partial arrays into one 256-bucket result. Empty input returns zero counts. A byte value 255 indexes bucket 255. Output must not depend on scheduling. A supplied enormous worker count must **not** spawn an unbounded number of threads; cap it at **16** and at the number of input bytes.

**Trace:** input `[1,1,2,255]` → bucket 1 = 2, bucket 2 = 1, bucket 255 = 1, all others zero, independent of the number of worker threads.

**APIs:** `std::thread::scope`, `spawn`, `join`, `slice::chunks`, `usize::from`, `Option`.

**Failure clinic:** Using a single shared counter array with unsynchronized writes is a data race (and rejected by safe Rust unless interior synchronization is introduced). Sharing a `Mutex` for every increment may make parallel execution slower; independent partials are often simpler and faster. A caller request for 100,000 workers should not result in 100,000 threads.

**Test:** `cargo test --test mastery a02_parallel`.

## A03 — Apply all-or-nothing balance changes

**Signature:** `apply_changes(balances: &mut BTreeMap<String,u64>, deltas: &[(String,i64)]) -> Result<(),usize>`

**Contract:** Each delta modifies an **existing** key only; no implicit account creation. Apply deltas in input order, including repeated keys. Negative results (underflow) and results over `u64::MAX` (overflow) are invalid. Return the **zero-based index of the first failing delta**. On **any** failure, the original map must be byte-for-byte equivalent in content to the entry snapshot; partial modifications are forbidden. An empty delta list succeeds without changing the map.

**Trace:** balance Ada=6 and deltas [(-2), (+5)] produce Ada=9. If the second delta targets a nonexistent key, neither update may be committed.

**APIs:** `BTreeMap::clone`, `get_mut`, `i128::from`, `u64::try_from`, `?`, and a final `*balances = staged` commit.

**Failure clinic:** Updating the input map directly creates partial writes on a later failure. Using `i64` for the current balance cannot represent `u64::MAX`. Mutating the map in the middle of validation makes reasoning and recovery much harder.

**Scope note:** This is an in-memory educational transaction. It does not implement durable storage, database isolation, logging or crash recovery.

**Test:** `cargo test --test mastery a03_atomic`.

## A04 — Implement a bounded lock-free reservation counter

**Signature:** `reserve_capacity(current: &AtomicUsize, limit: usize, amount: usize) -> Result<usize, ReservationError>`

**Contract:** Increase the current count by `amount` without ever exceeding `limit` or wrapping `usize`. On success return the count **before** reservation. On error return `CapacityExceeded` and leave the counter unchanged. A zero-sized reservation succeeds and returns the observed count without mutation, provided the count is already within the limit. Multiple concurrent requests must never allocate more capacity than the limit.

**Trace:** current=5, limit=10, request=3 → `Ok(5)` and new count=8. Another request of 3 → `Err(CapacityExceeded)` and count remains 8.

**APIs:** `AtomicUsize::load`, `checked_add`, `compare_exchange_weak`, `Ordering::AcqRel`, `Ordering::Acquire`. Retry the compare-exchange loop after contention; do not confuse a CAS failure with lack of capacity.

**Failure clinic:** A load followed by an unconditional store can oversubscribe the limit when multiple threads read the same old count. `fetch_add` without a bound check can overflow or commit an invalid increment. `compare_exchange_weak` may fail spuriously, so retry correctly.

**Scope note:** This is a thread-local process synchronization exercise. It is **not** persistent, globally distributed, cancellation-safe ticketing or authorization. Real reservations need expiration, reconciliation, idempotency and durable storage.

**Test:** `cargo test --test mastery a04_atomic`.

## Review and debugging exercise

Draw two concurrent reservation threads observing count=19 when limit=20. Show why both could succeed under a naive load/store, then explain how exactly one compare-exchange can commit its reservation. For the frame reader, produce a truth table for clean EOF, incomplete header, incomplete payload, oversized payload, and frame count exceeded. For the ledger, show a two-step failure with complete rollback.

## Advanced portfolio milestone

Write an **offline reliability review** composed of four parts: framing error table, histogram invariants (sum of 256 counts equals input length), transaction rollback proof with first-error index, and a concurrent reservation execution trace. Include real passing test commands, at least one intentionally rejected case per task, and a paragraph on what a production version would still require.

**Completion:** All four Advanced mastery tests pass in default mode, and you can explain the tradeoffs between thread-local atomicity, map snapshots and durable transactions. Finish with the [full portfolio](../PORTFOLIO.md) and [assessment](../ASSESSMENT.md).
