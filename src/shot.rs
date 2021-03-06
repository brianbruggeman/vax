//!  Shot structures

#[derive(Clone, Debug)]
enum ShotVariant {
    // Pfizer shots  - 2 dosage - RNA
    Pfizer,
    // Moderna shots  - 2 dosage - RNA
    Moderna,
    // Jj shot - 1 dosage - traditional
    Jj,
}

#[derive(Clone, Debug)]
pub struct ShotData {
    variant: ShotVariant,
    location: String,
}