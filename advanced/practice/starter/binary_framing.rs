/// Read the first two bytes as a big-endian u16. Ignore trailing bytes.
pub fn word(bytes: &[u8]) -> Option<u16> {
    todo!("Implement word")
}

/// Decode one two-byte length-prefixed payload and return the rest.
pub fn frame(data: &[u8]) -> Option<(&[u8], &[u8])> {
    todo!("Implement frame")
}

/// Prefix a payload with its length. Reject lengths greater than u16::MAX.
pub fn encode(data: &[u8]) -> Option<Vec<u8>> {
    todo!("Implement encode")
}

/// Decode an entire batch. For a truncated frame report its starting byte offset.
pub fn frames(mut data: &[u8]) -> Result<Vec<&[u8]>, usize> {
    todo!("Implement frames")
}

/// Compute the additive complement so payload plus checksum sums to zero modulo 256.
pub fn checksum(data: &[u8]) -> u8 {
    todo!("Implement checksum")
}
