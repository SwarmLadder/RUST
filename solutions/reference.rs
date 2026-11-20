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

use crate::Session;
use std::collections::BTreeMap;

pub fn sessions(log: &str) -> Result<Vec<Session>, (usize, EntryError)> {
    let mut rows = Vec::new();
    for (index, line) in log.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (topic, minutes) = parse_entry(line).map_err(|err| (index + 1, err))?;
        rows.push(Session { topic: topic.to_owned(), minutes });
    }
    Ok(rows)
}

pub fn totals(rows: &[Session]) -> BTreeMap<&str, u64> {
    let mut result = BTreeMap::new();
    for row in rows {
        *result.entry(row.topic.as_str()).or_insert(0) += u64::from(row.minutes);
    }
    result
}

pub fn render(by_topic: &BTreeMap<&str, u64>) -> String {
    if by_topic.is_empty() {
        return String::from("No sessions.\n");
    }
    let mut text = String::new();
    for (topic, minutes) in by_topic {
        text.push_str(&format!("{topic}: {minutes} min\n"));
    }
    text
}

pub fn report(log: &str) -> Result<String, String> {
    let rows = sessions(log).map_err(|(line, err)| {
        let reason = match err {
            EntryError::MissingSeparator => "expected a comma",
            EntryError::EmptyTitle => "topic is blank",
            EntryError::InvalidMinutes => "minutes must be a nonnegative u32",
        };
        format!("line {line}: {reason}")
    })?;
    Ok(render(&totals(&rows)))
}
