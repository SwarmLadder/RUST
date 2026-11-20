use super::Parcel;

/// Construct a Parcel with the given pieces and unit_grams.
pub fn parcel(pieces: u16, grams: u16) -> Parcel {
    Parcel {
        pieces,
        unit_grams: grams,
    }
}

/// Compute total parcel weight in grams.
pub fn weight(p: &Parcel) -> u32 {
    u32::from(p.pieces) * u32::from(p.unit_grams)
}

/// Decide whether the first parcel strictly outweighs the second.
pub fn heavier(a: &Parcel, b: &Parcel) -> bool {
    weight(a) > weight(b)
}

/// Consume a parcel and return quantity then unit weight.
pub fn unpack(p: Parcel) -> (u16, u16) {
    (p.pieces, p.unit_grams)
}

/// Combine quantities only for equal unit weights and a representable total.
pub fn combine(a: Parcel, b: Parcel) -> Option<Parcel> {
    if a.unit_grams != b.unit_grams {
        return None;
    }
    Some(Parcel {
        pieces: a.pieces.checked_add(b.pieces)?,
        ..a
    })
}
