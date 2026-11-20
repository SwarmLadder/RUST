/// Read lines incrementally. Count INFO, WARN and ERROR by first whitespace-delimited token, in that order. Ignore other lines. Propagate read and UTF-8 errors.
pub fn events<R: std::io::BufRead>(reader: R) -> std::io::Result<[u64; 3]> {
    let mut counts = [0; 3];
    for line in reader.lines() {
        let line = line?;
        match line.split_whitespace().next() {
            Some("INFO") => counts[0] += 1,
            Some("WARN") => counts[1] += 1,
            Some("ERROR") => counts[2] += 1,
            _ => (),
        }
    }
    Ok(counts)
}

/// Reject zero workers. Divide nonempty input across at most the requested workers using scoped threads. Each worker owns a fixed-size array. Join and reduce without shared mutable counters.
pub fn histogram(bytes: &[u8], workers: usize) -> Option<[usize; 256]> {
    if workers == 0 {
        return None;
    }
    let mut totals = [0; 256];
    if bytes.is_empty() {
        return Some(totals);
    }
    let n = bytes.len();
    let size = n / workers + usize::from(n % workers != 0);
    std::thread::scope(|scope| {
        let handles: Vec<_> = bytes
            .chunks(size)
            .map(|chunk| {
                scope.spawn(move || {
                    let mut local = [0usize; 256];
                    for &v in chunk {
                        local[v as usize] += 1
                    }
                    local
                })
            })
            .collect();
        for handle in handles {
            for (a, b) in totals.iter_mut().zip(handle.join().unwrap()) {
                *a += b;
            }
        }
    });
    Some(totals)
}

/// Reject invalid account indices, self-transfers, insufficient funds and recipient overflow. Report the first zero-based transfer index. Successful transfers affect later ones. Commit only after every transfer succeeds.
pub fn settle(accounts: &mut [u32], transfers: &[(usize, usize, u32)]) -> Result<(), usize> {
    let mut next = accounts.to_vec();
    for (i, &(from, to, n)) in transfers.iter().enumerate() {
        if from >= next.len() || to >= next.len() || from == to {
            return Err(i);
        }
        next[from] = next[from].checked_sub(n).ok_or(i)?;
        next[to] = next[to].checked_add(n).ok_or(i)?;
    }
    accounts.copy_from_slice(&next);
    Ok(())
}
