use super::Label;

/// Use the supplied Label trait to render one label surrounded by brackets.
pub fn render<T: Label>(x: &T) -> String {
    todo!("Implement render")
}

/// Render each raw label in order, allowing mixed input types.
pub fn labels(xs: &[&dyn Label]) -> Vec<String> {
    todo!("Implement labels")
}

/// Join bracketed labels with a comma and no spaces.
pub fn render_all<T: Label>(xs: &[T]) -> String {
    todo!("Implement render_all")
}

/// Compare labels across different concrete types.
pub fn same_label(a: &dyn Label, b: &dyn Label) -> bool {
    todo!("Implement same_label")
}

/// Choose the first longest label by Unicode scalar count.
pub fn longest_label(xs: &[&dyn Label]) -> Option<String> {
    todo!("Implement longest_label")
}
