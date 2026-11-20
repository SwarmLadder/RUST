/// Match (), [] and {} with proper nesting. Ignore other characters.
pub fn balanced(s: &str) -> bool {
    todo!("Implement balanced")
}

/// Some pushes a value. None undoes the last surviving push. Empty undo is harmless.
pub fn undo(xs: &[Option<i32>]) -> Vec<i32> {
    todo!("Implement undo")
}

/// Emit one job name per turn. Requeue unfinished jobs and skip zero-turn jobs.
pub fn round_robin(jobs: &[(&str, u8)]) -> Vec<String> {
    todo!("Implement round_robin")
}

/// Keep at most the final capacity events, oldest first.
pub fn recent(xs: &[u32], capacity: usize) -> Vec<u32> {
    todo!("Implement recent")
}

/// Evaluate postfix i32 tokens with checked + and -. Require exactly one final value.
pub fn evaluate(tokens: &[&str]) -> Option<i32> {
    todo!("Implement evaluate")
}
