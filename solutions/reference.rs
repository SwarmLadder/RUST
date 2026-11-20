use crate::{EntryError, Summary};

pub fn blocks(minutes: u32) -> (u32, u32) {
    (minutes / 25, minutes % 25)
}

pub fn title(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

pub fn parse_entry(line: &str) -> Result<(&str, u32), EntryError> {
    let (name, minutes) = line.split_once(',').ok_or(EntryError::MissingSeparator)?;
    let name = title(name).ok_or(EntryError::EmptyTitle)?;
    let minutes = minutes.trim().parse::<u32>().map_err(|_| EntryError::InvalidMinutes)?;
    Ok((name, minutes))
}

pub fn summarize(log: &str) -> Result<Summary, (usize, EntryError)> {
    let mut summary = Summary { sessions: 0, minutes: 0 };
    for (index, line) in log.lines().enumerate() {
        if line.trim().is_empty() { continue; }
        let (_, minutes) = parse_entry(line).map_err(|err| (index + 1, err))?;
        summary.sessions += 1;
        summary.minutes += u64::from(minutes);
    }
    Ok(summary)
}
