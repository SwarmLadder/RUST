use super::Expr;

/// Count all literals and operators.
pub fn nodes(e: &Expr) -> usize {
    todo!("Implement nodes")
}

/// Return tree height counting the root as one.
pub fn depth(e: &Expr) -> usize {
    todo!("Implement depth")
}

/// Evaluate using checked arithmetic at every internal node.
pub fn compute(e: &Expr) -> Option<i32> {
    todo!("Implement compute")
}

/// Emit literals and operators in postfix order, separated by single spaces.
pub fn postfix(e: &Expr) -> String {
    todo!("Implement postfix")
}

/// Consume the tree and transform every literal, preserving operators and shape.
pub fn map_literals(e: Expr, f: &impl Fn(i32) -> i32) -> Expr {
    todo!("Implement map_literals")
}
