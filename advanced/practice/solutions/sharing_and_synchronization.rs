/// Return two handles to one allocation.
pub fn share<T>(value: T) -> (std::sync::Arc<T>, std::sync::Arc<T>) {
    let a = std::sync::Arc::new(value);
    (a.clone(), a)
}

/// Atomically increment a standalone counter and return its previous value. Wrap on overflow.
pub fn ticket(counter: &std::sync::atomic::AtomicUsize) -> usize {
    counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Append only if try_lock succeeds immediately. Report contention or poison as false.
pub fn try_append(xs: &std::sync::Mutex<Vec<i32>>, v: i32) -> bool {
    if let Ok(mut xs) = xs.try_lock() {
        xs.push(v);
        true
    } else {
        false
    }
}

/// Use a scoped worker for one half and the caller for the other. Combine optional maxima.
pub fn parallel_max(xs: &[i32]) -> Option<i32> {
    let (a, b) = xs.split_at(xs.len() / 2);
    std::thread::scope(|s| {
        let t = s.spawn(|| a.iter().copied().max());
        let right = b.iter().copied().max();
        t.join().unwrap().max(right)
    })
}

/// Spawn each job in a scope, join all, preserve submission order and return the first panic in that order.
pub fn ordered_jobs<T: Send>(jobs: Vec<impl FnOnce() -> T + Send>) -> std::thread::Result<Vec<T>> {
    std::thread::scope(|s| {
        let handles: Vec<_> = jobs.into_iter().map(|f| s.spawn(f)).collect();
        let results: Vec<_> = handles.into_iter().map(|h| h.join()).collect();
        results.into_iter().collect()
    })
}
