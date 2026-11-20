# Binary framing

## Decoding length-prefixed records

Network-order integers use big-endian bytes. Parse headers before slicing payloads and check for truncation before advancing. Borrowing payload slices allows a batch of frames to share one input allocation.

**Worked trace.** The bytes 00 02 41 42 represent a two-byte payload AB. A following zero-length frame still consumes its two-byte header.

## Try it

```rust
let packet = [0, 2, 65, 66];
let len = usize::from(u16::from_be_bytes([packet[0], packet[1]]));
assert_eq!(packet.get(2..2 + len), Some(&b"AB"[..]));
```

Change the length to 3 while keeping the same bytes. Predict the return value of get.

## Exercises

Edit [the starter functions](starter/binary_framing.rs). The [acceptance tests](../../tests/advanced_practice.rs) show example inputs and expected results.

### `word`

Read the first two bytes as a big-endian u16. Ignore trailing bytes.

`cargo test --test advanced_practice word -- --exact`

### `frame`

Decode one two-byte length-prefixed payload and return the rest.

`cargo test --test advanced_practice frame -- --exact`

### `encode`

Prefix a payload with its length. Reject lengths greater than u16::MAX.

`cargo test --test advanced_practice encode -- --exact`

### `frames`

Decode an entire batch. For a truncated frame report its starting byte offset.

`cargo test --test advanced_practice frames -- --exact`

### `checksum`

Compute the additive complement so payload plus checksum sums to zero modulo 256.

`cargo test --test advanced_practice checksum -- --exact`

## Review questions

1. How many bytes does an empty frame occupy?
2. Which offset identifies a truncated second frame after a three-byte first frame?

<details><summary>Check your reasoning</summary>

1. Two bytes for its zero length.
2. Three, the start of the second header.

</details>

[Compare worked solutions](solutions/binary_framing.rs) · [Return to the level guide](../README.md)
