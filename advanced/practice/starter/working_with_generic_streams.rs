/// Read exactly four bytes as a little-endian u32. Propagate a short read.
pub fn read_number<R: std::io::Read>(mut r: R) -> std::io::Result<u32> {
    todo!("Implement read_number")
}

/// Copy at most n bytes, returning the count and leaving later bytes unread.
pub fn bounded<R: std::io::Read, W: std::io::Write>(
    r: &mut R,
    w: &mut W,
    n: u64,
) -> std::io::Result<u64> {
    todo!("Implement bounded")
}

/// Write a two-byte big-endian length and complete payload. Reject oversized input with InvalidInput before writing.
pub fn write_record<W: std::io::Write>(mut w: W, data: &[u8]) -> std::io::Result<()> {
    todo!("Implement write_record")
}

/// Return the last nonblank line with its original surrounding spaces, excluding the line terminator.
pub fn last_nonblank<R: std::io::BufRead>(r: R) -> std::io::Result<Option<String>> {
    todo!("Implement last_nonblank")
}

/// Inspect the next byte without consuming it. Empty input returns None.
pub fn peek<R: std::io::BufRead>(r: &mut R) -> std::io::Result<Option<u8>> {
    todo!("Implement peek")
}
