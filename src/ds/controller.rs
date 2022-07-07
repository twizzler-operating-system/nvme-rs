use modular_bitfield::prelude::*;

#[bitfield(bits = 16)]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct ControllerId(u16);
