use super::Revision;

/// Require the expected version and a representable next version. Return the updated state.
pub fn apply(s: Revision, expected: u32, value: i32) -> Option<Revision> {
    if expected != s.version {
        return None;
    }
    Some(Revision {
        version: s.version.checked_add(1)?,
        value,
    })
}

/// Apply edits sequentially or report the zero-based index of the first rejected edit.
pub fn replay(mut s: Revision, edits: &[(u32, i32)]) -> Result<Revision, usize> {
    for (i, &(v, x)) in edits.iter().enumerate() {
        s = apply(s, v, x).ok_or(i)?
    }
    Ok(s)
}

/// Apply a batch atomically to the provided state. A rejected edit leaves both fields intact.
pub fn commit(s: &mut Revision, edits: &[(u32, i32)]) -> Result<(), usize> {
    *s = replay(*s, edits)?;
    Ok(())
}

/// Return the opening state followed by each successful state. Reject an invalid history with its edit index.
pub fn history(mut s: Revision, edits: &[(u32, i32)]) -> Result<Vec<Revision>, usize> {
    let mut out = vec![s];
    for (i, &(v, x)) in edits.iter().enumerate() {
        s = apply(s, v, x).ok_or(i)?;
        out.push(s)
    }
    Ok(out)
}

/// Find the state at a version in a history sorted by strictly increasing version. Use binary search.
pub fn rollback(states: &[Revision], version: u32) -> Option<Revision> {
    states
        .binary_search_by_key(&version, |s| s.version)
        .ok()
        .map(|i| states[i])
}
