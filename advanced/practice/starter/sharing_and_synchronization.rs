/// Return two handles to one allocation.
pub fn share<T>(value: T) -> (std::sync::Arc<T>, std::sync::Arc<T>) {
    todo!("Implement share")
}

/// Atomically increment a standalone counter and return its previous value. Wrap on overflow.
pub fn ticket(counter: &std::sync::atomic::AtomicUsize) -> usize {
    todo!("Implement ticket")
}

/// Append only if try_lock succeeds immediately. Report contention or poison as false.
pub fn try_append(xs: &std::sync::Mutex<Vec<i32>>, v: i32) -> bool {
    todo!("Implement try_append")
}

/// Use a scoped worker for one half and the caller for the other. Combine optional maxima.
pub fn parallel_max(xs: &[i32]) -> Option<i32> {
    todo!("Implement parallel_max")
}

/// Spawn each job in a scope, join all, preserve submission order and return the first panic in that order.
pub fn ordered_jobs<T: Send>(jobs: Vec<impl FnOnce() -> T + Send>) -> std::thread::Result<Vec<T>> {
    todo!("Implement ordered_jobs")
}
