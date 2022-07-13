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

impl From<ControllerId> for u16 {
    fn from(c: ControllerId) -> Self {
        c.get_0()
    }
}

pub mod properties;
