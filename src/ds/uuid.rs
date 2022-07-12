use modular_bitfield::prelude::*;

#[bitfield(bits = 7)]
#[derive(BitfieldSpecifier)]
pub struct UuidIndex(B7);

impl From<Option<UuidIndex>> for UuidIndex {
    fn from(o: Option<UuidIndex>) -> Self {
        match o {
            Some(u) => u,
            None => UuidIndex::new().with_0(0),
        }
    }
}
