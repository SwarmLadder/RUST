/// Read the first two bytes as a big-endian u16. Ignore trailing bytes.
pub fn word(bytes: &[u8]) -> Option<u16> {
    Some(u16::from_be_bytes(bytes.get(..2)?.try_into().ok()?))
}

/// Decode one two-byte length-prefixed payload and return the rest.
pub fn frame(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let n = usize::from(word(data)?);
    let rest = &data[2..];
    if n > rest.len() {
        return None;
    }
    Some(rest.split_at(n))
}

/// Prefix a payload with its length. Reject lengths greater than u16::MAX.
pub fn encode(data: &[u8]) -> Option<Vec<u8>> {
    let n = u16::try_from(data.len()).ok()?;
    let mut v = n.to_be_bytes().to_vec();
    v.extend_from_slice(data);
    Some(v)
}

/// Decode an entire batch. For a truncated frame report its starting byte offset.
pub fn frames(mut data: &[u8]) -> Result<Vec<&[u8]>, usize> {
    let total = data.len();
    let mut out = vec![];
    while !data.is_empty() {
        let (p, r) = frame(data).ok_or(total - data.len())?;
        out.push(p);
        data = r
    }
    Ok(out)
}

/// Compute the additive complement so payload plus checksum sums to zero modulo 256.
pub fn checksum(data: &[u8]) -> u8 {
    0u8.wrapping_sub(data.iter().copied().fold(0, u8::wrapping_add))
}
