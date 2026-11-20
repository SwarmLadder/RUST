/// Find greatest common divisor by repeated remainder. Define gcd(0,0) as 0.
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

/// Count decimal digits, including the single digit in zero.
pub fn digits(mut n: u32) -> u32 {
    let mut k = 1;
    while n >= 10 {
        n /= 10;
        k += 1
    }
    k
}

/// Multiply 1 through n with overflow detection.
pub fn factorial(n: u8) -> Option<u64> {
    (1..=n).try_fold(1u64, |a, v| a.checked_mul(u64::from(v)))
}

/// List positive powers of two starting at 1 through limit, inclusively.
pub fn powers(limit: u32) -> Vec<u32> {
    let mut out = vec![];
    let mut n = 1u32;
    while n <= limit {
        out.push(n);
        match n.checked_mul(2) {
            Some(v) => n = v,
            None => break,
        }
    }
    out
}

/// Sum decimal digits using division and remainder.
pub fn digit_sum(mut n: u32) -> u32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10
    }
    s
}
