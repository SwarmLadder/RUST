use super::Signal;

/// Cycle Red → Green → Amber → Red.
pub fn next(s: Signal) -> Signal {
    todo!("Implement next")
}

/// Recognize exactly red, green and amber in lowercase.
pub fn signal(s: &str) -> Option<Signal> {
    todo!("Implement signal")
}

/// Permit movement only on Green.
pub fn may_go(s: Signal) -> bool {
    todo!("Implement may_go")
}

/// Advance n steps without looping n times.
pub fn advance(s: Signal, n: u32) -> Signal {
    todo!("Implement advance")
}

/// Count green signals in a trace.
pub fn green_count(signals: &[Signal]) -> usize {
    todo!("Implement green_count")
}
