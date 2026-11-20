/// Check that every neighbor is a valid vertex index.
pub fn valid_graph(g: &[Vec<usize>]) -> bool {
    todo!("Implement valid_graph")
}

/// Reverse every edge in source traversal order. Reject invalid graphs. Preserve parallel edges.
pub fn transpose(g: &[Vec<usize>]) -> Option<Vec<Vec<usize>>> {
    todo!("Implement transpose")
}

/// Return fewest-edge distances. Reject invalid graph or start. Unreachable vertices have None.
pub fn distances(g: &[Vec<usize>], start: usize) -> Option<Vec<Option<usize>>> {
    todo!("Implement distances")
}

/// Return first shortest path in neighbor order. Reject invalid or unreachable requests.
pub fn path(g: &[Vec<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
    todo!("Implement path")
}

/// Return a topological order, choosing smallest currently ready vertex. Reject invalid or cyclic graphs.
pub fn topological(g: &[Vec<usize>]) -> Option<Vec<usize>> {
    todo!("Implement topological")
}
