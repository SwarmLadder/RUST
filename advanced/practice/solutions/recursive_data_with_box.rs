use super::Expr;

/// Count all literals and operators.
pub fn nodes(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 1,
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + nodes(a) + nodes(b),
    }
}

/// Return tree height counting the root as one.
pub fn depth(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 1,
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + depth(a).max(depth(b)),
    }
}

/// Evaluate using checked arithmetic at every internal node.
pub fn compute(e: &Expr) -> Option<i32> {
    match e {
        Expr::Lit(v) => Some(*v),
        Expr::Add(a, b) => compute(a)?.checked_add(compute(b)?),
        Expr::Mul(a, b) => compute(a)?.checked_mul(compute(b)?),
    }
}

/// Emit literals and operators in postfix order, separated by single spaces.
pub fn postfix(e: &Expr) -> String {
    match e {
        Expr::Lit(v) => v.to_string(),
        Expr::Add(a, b) => format!("{} {} +", postfix(a), postfix(b)),
        Expr::Mul(a, b) => format!("{} {} *", postfix(a), postfix(b)),
    }
}

/// Consume the tree and transform every literal, preserving operators and shape.
pub fn map_literals(e: Expr, f: &impl Fn(i32) -> i32) -> Expr {
    match e {
        Expr::Lit(v) => Expr::lit(f(v)),
        Expr::Add(a, b) => Expr::sum(map_literals(*a, f), map_literals(*b, f)),
        Expr::Mul(a, b) => Expr::product(map_literals(*a, f), map_literals(*b, f)),
    }
}
