use super::Label;

/// Use the supplied Label trait to render one label surrounded by brackets.
pub fn render<T: Label>(x: &T) -> String {
    format!("[{}]", x.label())
}

/// Render each raw label in order, allowing mixed input types.
pub fn labels(xs: &[&dyn Label]) -> Vec<String> {
    xs.iter().map(|x| x.label()).collect()
}

/// Join bracketed labels with a comma and no spaces.
pub fn render_all<T: Label>(xs: &[T]) -> String {
    xs.iter().map(render).collect::<Vec<_>>().join(",")
}

/// Compare labels across different concrete types.
pub fn same_label(a: &dyn Label, b: &dyn Label) -> bool {
    a.label() == b.label()
}

/// Choose the first longest label by Unicode scalar count.
pub fn longest_label(xs: &[&dyn Label]) -> Option<String> {
    let mut best = None;
    let mut n = 0;
    for x in xs {
        let s = x.label();
        let len = s.chars().count();
        if best.is_none() || len > n {
            n = len;
            best = Some(s)
        }
    }
    best
}
