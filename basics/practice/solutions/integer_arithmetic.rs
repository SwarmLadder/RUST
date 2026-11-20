/// Return whole elapsed minutes.
pub fn minutes(s: u32) -> u32 {
    s / 60
}

/// Split elapsed seconds into hours, minutes, seconds. Hours may exceed 23.
pub fn clock(s: u32) -> (u32, u32, u32) {
    (s / 3600, s / 60 % 60, s % 60)
}

/// Calculate rectangular area without losing precision.
pub fn area(w: u16, h: u16) -> u32 {
    u32::from(w) * u32::from(h)
}

/// Return the final decimal digit, ignoring the sign.
pub fn last_digit(n: i32) -> u32 {
    n.unsigned_abs() % 10
}

/// Round partial minutes up, without overflowing.
pub fn ceil_minutes(s: u32) -> u32 {
    s / 60 + u32::from(s % 60 != 0)
}
