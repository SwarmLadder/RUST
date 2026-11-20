#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingEquals,
    EmptyKey,
    InvalidValue,
}

pub trait Label {
    fn label(&self) -> String;
}
impl Label for String {
    fn label(&self) -> String {
        self.clone()
    }
}
impl Label for u32 {
    fn label(&self) -> String {
        self.to_string()
    }
}
