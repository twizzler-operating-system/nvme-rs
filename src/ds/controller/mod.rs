use modular_bitfield::prelude::*;

#[bitfield(bits = 16)]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct ControllerId(u16);

impl From<u16> for ControllerId {
    fn from(x: u16) -> Self {
        ControllerId::new().with_0(x)
    }
}



pub mod properties;