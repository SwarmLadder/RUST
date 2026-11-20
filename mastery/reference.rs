//! Fully worked reference algorithms. Selected only with --features reference.
use super::{ConfigError, PortError, ReservationError};
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn parse_port(input: &str) -> Result<u16, PortError> {
    let value = input.trim();
    if value.is_empty() {
        return Err(PortError::Empty);
    }
    if !value.bytes().all(|ch| ch.is_ascii_digit()) {
        return Err(PortError::InvalidCharacters);
    }
    match value.parse::<u16>() {
        Ok(port) if port != 0 => Ok(port),
        _ => Err(PortError::OutOfRange),
    }
}

pub fn unicode_window(input: &str, start: usize, count: usize) -> Option<&str> {
    let end = start.checked_add(count)?;
    let boundaries: Vec<usize> = input
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(input.len()))
        .collect();
    if end >= boundaries.len() {
        return None;
    }
    Some(&input[boundaries[start]..boundaries[end]])
}

pub fn mean_truncated(values: &[i64]) -> Option<i64> {
    if values.is_empty() {
        return None;
    }
    let sum = values.iter().map(|&v| i128::from(v)).sum::<i128>();
    Some((sum / values.len() as i128) as i64)
}

pub fn word_frequency(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for part in text.split_whitespace() {
        let normalized = part
            .trim_matches(|ch: char| ch.is_ascii_punctuation())
            .to_lowercase();
        if !normalized.is_empty() {
            let count = counts.entry(normalized).or_insert(0usize);
            *count = count.saturating_add(1);
        }
    }
    counts
}

pub fn parse_settings(text: &str) -> Result<BTreeMap<String, String>, ConfigError> {
    let mut settings = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or(ConfigError::MissingEquals(number))?;
        let key = key.trim();
        if key.is_empty() {
            return Err(ConfigError::EmptyKey(number));
        }
        if settings.contains_key(key) {
            return Err(ConfigError::DuplicateKey(number, key.to_owned()));
        }
        settings.insert(key.to_owned(), value.trim().to_owned());
    }
    Ok(settings)
}

pub fn by_initial(words: &[String]) -> BTreeMap<char, Vec<&str>> {
    let mut groups = BTreeMap::new();
    for word in words {
        let trimmed = word.trim();
        if let Some(letter) = trimmed.chars().next() {
            groups.entry(letter).or_insert_with(Vec::new).push(trimmed);
        }
    }
    groups
}

pub fn normalize_ranges(ranges: &[(u32, u32)]) -> Result<Vec<(u32, u32)>, usize> {
    for (index, &(start, end)) in ranges.iter().enumerate() {
        if start > end {
            return Err(index);
        }
    }
    let mut sorted = ranges.to_vec();
    sorted.sort_unstable();
    let mut merged: Vec<(u32, u32)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut() {
            if start <= last.1.saturating_add(1) {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    Ok(merged)
}

pub fn top_k_scores(entries: &[(String, i64)], k: usize) -> Vec<(String, i64)> {
    let mut ranked = entries.to_vec();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.truncate(k);
    ranked
}

pub fn read_frames<R: Read>(
    mut reader: R,
    max_payload: usize,
    max_frames: usize,
) -> io::Result<Vec<Vec<u8>>> {
    let mut frames = Vec::new();
    loop {
        let mut header = [0u8; 2];
        match reader.read(&mut header[..1])? {
            0 => return Ok(frames),
            _ => {}
        }
        if frames.len() == max_frames {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "frame limit exceeded"));
        }
        reader.read_exact(&mut header[1..])?;
        let length = u16::from_be_bytes(header) as usize;
        if length > max_payload {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "payload limit exceeded"));
        }
        let mut payload = vec![0u8; length];
        reader.read_exact(&mut payload)?;
        frames.push(payload);
    }
}

pub fn parallel_histogram(bytes: &[u8], workers: usize) -> Option<[usize; 256]> {
    if workers == 0 {
        return None;
    }
    if bytes.is_empty() {
        return Some([0usize; 256]);
    }
    // Bound task creation even for a caller-supplied enormous worker count.
    let workers = workers.min(bytes.len()).min(16);
    let chunk_size = bytes.len() / workers + usize::from(bytes.len() % workers != 0);
    std::thread::scope(|scope| {
        let handles: Vec<_> = bytes
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(move || {
                    let mut local = [0usize; 256];
                    for byte in chunk {
                        local[usize::from(*byte)] += 1;
                    }
                    local
                })
            })
            .collect();
        let mut counts = [0usize; 256];
        for handle in handles {
            let local = handle.join().ok()?;
            for (total, count) in counts.iter_mut().zip(local) {
                *total += count;
            }
        }
        Some(counts)
    })
}

pub fn apply_changes(
    balances: &mut BTreeMap<String, u64>,
    deltas: &[(String, i64)],
) -> Result<(), usize> {
    let mut updated = balances.clone();
    for (index, (key, delta)) in deltas.iter().enumerate() {
        let value = updated.get_mut(key).ok_or(index)?;
        let proposed = i128::from(*value) + i128::from(*delta);
        *value = u64::try_from(proposed).map_err(|_| index)?;
    }
    *balances = updated;
    Ok(())
}

pub fn reserve_capacity(
    current: &AtomicUsize,
    limit: usize,
    amount: usize,
) -> Result<usize, ReservationError> {
    loop {
        let before = current.load(Ordering::Acquire);
        let after = before
            .checked_add(amount)
            .ok_or(ReservationError::CapacityExceeded)?;
        if after > limit {
            return Err(ReservationError::CapacityExceeded);
        }
        if amount == 0 {
            return Ok(before);
        }
        if current
            .compare_exchange_weak(before, after, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Ok(before);
        }
    }
}
