use super::Signal;

/// Cycle Red → Green → Amber → Red.
pub fn next(s: Signal) -> Signal {
    match s {
        Signal::Red => Signal::Green,
        Signal::Green => Signal::Amber,
        Signal::Amber => Signal::Red,
    }
}

/// Recognize exactly red, green and amber in lowercase.
pub fn signal(s: &str) -> Option<Signal> {
    match s {
        "red" => Some(Signal::Red),
        "green" => Some(Signal::Green),
        "amber" => Some(Signal::Amber),
        _ => None,
    }
}

/// Permit movement only on Green.
pub fn may_go(s: Signal) -> bool {
    matches!(s, Signal::Green)
}

/// Advance n steps without looping n times.
pub fn advance(s: Signal, n: u32) -> Signal {
    (0..n % 3).fold(s, |state, _| next(state))
}

/// Count green signals in a trace.
pub fn green_count(signals: &[Signal]) -> usize {
    signals
        .iter()
        .filter(|s| matches!(s, Signal::Green))
        .count()
}
