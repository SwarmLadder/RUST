/// Check that every neighbor is a valid vertex index.
pub fn valid_graph(g: &[Vec<usize>]) -> bool {
    g.iter().flatten().all(|&v| v < g.len())
}

/// Reverse every edge in source traversal order. Reject invalid graphs. Preserve parallel edges.
pub fn transpose(g: &[Vec<usize>]) -> Option<Vec<Vec<usize>>> {
    if !valid_graph(g) {
        return None;
    }
    let mut out = vec![vec![]; g.len()];
    for (i, ns) in g.iter().enumerate() {
        for &v in ns {
            out[v].push(i)
        }
    }
    Some(out)
}

/// Return fewest-edge distances. Reject invalid graph or start. Unreachable vertices have None.
pub fn distances(g: &[Vec<usize>], start: usize) -> Option<Vec<Option<usize>>> {
    if start >= g.len() || !valid_graph(g) {
        return None;
    }
    let mut d = vec![None; g.len()];
    let mut q = std::collections::VecDeque::from([start]);
    d[start] = Some(0);
    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if d[v].is_none() {
                d[v] = d[u].map(|n| n + 1);
                q.push_back(v)
            }
        }
    }
    Some(d)
}

/// Return first shortest path in neighbor order. Reject invalid or unreachable requests.
pub fn path(g: &[Vec<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
    if !valid_graph(g) || start >= g.len() || end >= g.len() {
        return None;
    }
    let mut p = vec![None; g.len()];
    let mut q = std::collections::VecDeque::from([start]);
    p[start] = Some(start);
    while let Some(u) = q.pop_front() {
        if u == end {
            break;
        }
        for &v in &g[u] {
            if p[v].is_none() {
                p[v] = Some(u);
                q.push_back(v)
            }
        }
    }
    let mut out = vec![end];
    let mut i = end;
    while i != start {
        i = p[i]?;
        out.push(i)
    }
    out.reverse();
    Some(out)
}

/// Return a topological order, choosing smallest currently ready vertex. Reject invalid or cyclic graphs.
pub fn topological(g: &[Vec<usize>]) -> Option<Vec<usize>> {
    if !valid_graph(g) {
        return None;
    }
    let mut deg = vec![0; g.len()];
    for &v in g.iter().flatten() {
        deg[v] += 1
    }
    let mut ready: std::collections::BTreeSet<_> = (0..g.len()).filter(|&i| deg[i] == 0).collect();
    let mut out = vec![];
    while let Some(v) = ready.pop_first() {
        out.push(v);
        for &w in &g[v] {
            deg[w] -= 1;
            if deg[w] == 0 {
                ready.insert(w);
            }
        }
    }
    (out.len() == g.len()).then_some(out)
}
