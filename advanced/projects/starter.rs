/// Read lines incrementally. Count INFO, WARN and ERROR by first whitespace-delimited token, in that order. Ignore other lines. Propagate read and UTF-8 errors.
pub fn events<R: std::io::BufRead>(reader: R) -> std::io::Result<[u64; 3]> {
    todo!("Implement events")
}

/// Reject zero workers. Divide nonempty input across at most the requested workers using scoped threads. Each worker owns a fixed-size array. Join and reduce without shared mutable counters.
pub fn histogram(bytes: &[u8], workers: usize) -> Option<[usize; 256]> {
    todo!("Implement histogram")
}

/// Reject invalid account indices, self-transfers, insufficient funds and recipient overflow. Report the first zero-based transfer index. Successful transfers affect later ones. Commit only after every transfer succeeds.
pub fn settle(accounts: &mut [u32], transfers: &[(usize, usize, u32)]) -> Result<(), usize> {
    todo!("Implement settle")
}
