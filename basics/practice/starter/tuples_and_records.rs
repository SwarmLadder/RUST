use super::Parcel;

/// Construct a Parcel with the given pieces and unit_grams.
pub fn parcel(pieces: u16, grams: u16) -> Parcel {
    todo!("Implement parcel")
}

/// Compute total parcel weight in grams.
pub fn weight(p: &Parcel) -> u32 {
    todo!("Implement weight")
}

/// Decide whether the first parcel strictly outweighs the second.
pub fn heavier(a: &Parcel, b: &Parcel) -> bool {
    todo!("Implement heavier")
}

/// Consume a parcel and return quantity then unit weight.
pub fn unpack(p: Parcel) -> (u16, u16) {
    todo!("Implement unpack")
}

/// Combine quantities only for equal unit weights and a representable total.
pub fn combine(a: Parcel, b: Parcel) -> Option<Parcel> {
    todo!("Implement combine")
}
