/// Return index of the first failing element, evaluating only through that element.
pub fn first_bad<T>(xs: &[T], mut ok: impl FnMut(&T) -> bool) -> Option<usize> {
    todo!("Implement first_bad")
}

/// Move each element through the supplied transformation in source order.
pub fn transform<T, U>(xs: Vec<T>, f: impl FnMut(T) -> U) -> Vec<U> {
    todo!("Implement transform")
}

/// Call at most n times. Return first success, otherwise last failure. Zero attempts returns None.
pub fn retry<T, E>(n: usize, mut action: impl FnMut() -> Result<T, E>) -> Option<Result<T, E>> {
    todo!("Implement retry")
}

/// Check whether every adjacent pair satisfies agrees.
pub fn adjacent<T>(xs: &[T], mut agrees: impl FnMut(&T, &T) -> bool) -> bool {
    todo!("Implement adjacent")
}

/// Stop at the first None, returning the last successful accumulator.
pub fn fold_until(xs: &[i32], start: i32, mut step: impl FnMut(i32, i32) -> Option<i32>) -> i32 {
    todo!("Implement fold_until")
}
