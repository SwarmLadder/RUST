/// Borrow the prefix before the first nonempty delimiter. Return None if absent or empty.
pub fn before<'a>(text: &'a str, delimiter: &str) -> Option<&'a str> {
    if delimiter.is_empty() {
        return None;
    }
    text.split_once(delimiter).map(|(s, _)| s)
}

/// Return a checked half-open byte slice. Reject non-boundaries and reversed ranges.
pub fn byte_slice(text: &str, start: usize, end: usize) -> Option<&str> {
    text.get(start..end)
}

/// Find the first matching key in semicolon-separated key=value pairs. Trim both sides and skip malformed pairs.
pub fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.split(';')
        .filter_map(|p| p.split_once('='))
        .find_map(|(k, v)| (k.trim() == key).then(|| v.trim()))
}

/// Count all physical lines, propagating invalid UTF-8 and other read errors.
pub fn line_count<R: std::io::BufRead>(reader: R) -> std::io::Result<usize> {
    let mut n = 0;
    for line in reader.lines() {
        line?;
        n += 1;
    }
    Ok(n)
}

/// Write every row followed by one newline and propagate write errors.
pub fn write_rows<W: std::io::Write>(mut out: W, rows: &[&str]) -> std::io::Result<()> {
    for row in rows {
        writeln!(out, "{row}")?;
    }
    Ok(())
}

/// Find the first line containing needle, returning its one-based number. Stop reading once found.
pub fn first_line<R: std::io::BufRead>(reader: R, needle: &str) -> std::io::Result<Option<usize>> {
    for (i, line) in reader.lines().enumerate() {
        if line?.contains(needle) {
            return Ok(Some(i + 1));
        }
    }
    Ok(None)
}

/// Return ceiling division, including zero items. Reject zero workers; avoid overflow.
pub fn chunk_size(items: usize, workers: usize) -> Option<usize> {
    if workers == 0 {
        None
    } else {
        Some(items / workers + usize::from(items % workers != 0))
    }
}

/// Split in half, sum one half on a scoped thread and the other on the caller, widening each item.
pub fn parallel_sum(xs: &[i32]) -> i64 {
    let (a, b) = xs.split_at(xs.len() / 2);
    std::thread::scope(|s| {
        let left = s.spawn(|| a.iter().map(|&v| i64::from(v)).sum::<i64>());
        let right = b.iter().map(|&v| i64::from(v)).sum::<i64>();
        left.join().unwrap() + right
    })
}

/// Collect all messages until every sender disconnects, preserving receive order.
pub fn drain<T>(rx: std::sync::mpsc::Receiver<T>) -> Vec<T> {
    rx.into_iter().collect()
}

/// Return the adjusted balance if representable, including i32::MIN.
pub fn apply_delta(balance: u32, delta: i32) -> Option<u32> {
    balance.checked_add_signed(delta)
}

/// Apply all changes atomically. Return the zero-based index of the first invalid delta.
pub fn batch(balance: &mut u32, changes: &[i32]) -> Result<(), usize> {
    let mut next = *balance;
    for (i, &change) in changes.iter().enumerate() {
        next = apply_delta(next, change).ok_or(i)?;
    }
    *balance = next;
    Ok(())
}

/// Reject unknown or identical accounts, insufficient funds, and destination overflow. Commit both balances together.
pub fn transfer(accounts: &mut [u32], from: usize, to: usize, amount: u32) -> bool {
    if from == to || from >= accounts.len() || to >= accounts.len() {
        return false;
    }
    let Some(a) = accounts[from].checked_sub(amount) else {
        return false;
    };
    let Some(b) = accounts[to].checked_add(amount) else {
        return false;
    };
    accounts[from] = a;
    accounts[to] = b;
    true
}
