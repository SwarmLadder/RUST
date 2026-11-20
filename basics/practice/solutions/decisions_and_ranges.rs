/// Return fare 0 through age 5, 2 for ages 6–17, 3 from age 65, otherwise 5.
pub fn fare(age: u8) -> u8 {
    match age {
        0..=5 => 0,
        6..=17 => 2,
        65.. => 3,
        _ => 5,
    }
}

/// Apply Gregorian leap-year rules.
pub fn leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Assign A for 90–100, B for 80–89, C below 80. Reject scores over 100.
pub fn band(score: u8) -> Option<char> {
    match score {
        0..=79 => Some('C'),
        80..=89 => Some('B'),
        90..=100 => Some('A'),
        _ => None,
    }
}

/// Include both endpoints even when supplied backwards.
pub fn inside(x: i32, a: i32, b: i32) -> bool {
    (a.min(b)..=a.max(b)).contains(&x)
}

/// Number quadrants counterclockwise from upper right. Return None on either axis.
pub fn quadrant(x: i32, y: i32) -> Option<u8> {
    match (x.signum(), y.signum()) {
        (1, 1) => Some(1),
        (-1, 1) => Some(2),
        (-1, -1) => Some(3),
        (1, -1) => Some(4),
        _ => None,
    }
}
