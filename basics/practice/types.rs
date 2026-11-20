#[derive(Debug, PartialEq, Eq)]
pub struct Parcel {
    pub pieces: u16,
    pub unit_grams: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Red,
    Amber,
    Green,
}
