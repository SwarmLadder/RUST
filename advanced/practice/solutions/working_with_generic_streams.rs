/// Read exactly four bytes as a little-endian u32. Propagate a short read.
pub fn read_number<R: std::io::Read>(mut r: R) -> std::io::Result<u32> {
    let mut b = [0; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

/// Copy at most n bytes, returning the count and leaving later bytes unread.
pub fn bounded<R: std::io::Read, W: std::io::Write>(
    r: &mut R,
    w: &mut W,
    n: u64,
) -> std::io::Result<u64> {
    std::io::copy(&mut std::io::Read::take(r, n), w)
}

/// Write a two-byte big-endian length and complete payload. Reject oversized input with InvalidInput before writing.
pub fn write_record<W: std::io::Write>(mut w: W, data: &[u8]) -> std::io::Result<()> {
    let n = u16::try_from(data.len())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
    w.write_all(&n.to_be_bytes())?;
    w.write_all(data)
}

/// Return the last nonblank line with its original surrounding spaces, excluding the line terminator.
pub fn last_nonblank<R: std::io::BufRead>(r: R) -> std::io::Result<Option<String>> {
    let mut last = None;
    for l in r.lines() {
        let l = l?;
        if !l.trim().is_empty() {
            last = Some(l)
        }
    }
    Ok(last)
}

/// Inspect the next byte without consuming it. Empty input returns None.
pub fn peek<R: std::io::BufRead>(r: &mut R) -> std::io::Result<Option<u8>> {
    Ok(r.fill_buf()?.first().copied())
}
