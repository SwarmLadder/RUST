/// Match (), [] and {} with proper nesting. Ignore other characters.
pub fn balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

/// Some pushes a value. None undoes the last surviving push. Empty undo is harmless.
pub fn undo(xs: &[Option<i32>]) -> Vec<i32> {
    let mut v = vec![];
    for &x in xs {
        match x {
            Some(i) => v.push(i),
            None => {
                v.pop();
            }
        }
    }
    v
}

/// Emit one job name per turn. Requeue unfinished jobs and skip zero-turn jobs.
pub fn round_robin(jobs: &[(&str, u8)]) -> Vec<String> {
    let mut q: std::collections::VecDeque<_> = jobs.iter().copied().filter(|j| j.1 > 0).collect();
    let mut v = vec![];
    while let Some((s, n)) = q.pop_front() {
        v.push(s.to_owned());
        if n > 1 {
            q.push_back((s, n - 1))
        }
    }
    v
}

/// Keep at most the final capacity events, oldest first.
pub fn recent(xs: &[u32], capacity: usize) -> Vec<u32> {
    xs[xs.len().saturating_sub(capacity)..].to_vec()
}

/// Evaluate postfix i32 tokens with checked + and -. Require exactly one final value.
pub fn evaluate(tokens: &[&str]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for &t in tokens {
        match t {
            "+" | "-" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(if t == "+" {
                    a.checked_add(b)?
                } else {
                    a.checked_sub(b)?
                })
            }
            _ => stack.push(t.parse().ok()?),
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
