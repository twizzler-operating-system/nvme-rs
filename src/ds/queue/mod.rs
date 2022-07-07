use modular_bitfield::bitfield;

pub mod comentry;
pub mod subentry;

use modular_bitfield::prelude::*;

#[bitfield]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct QueueSize(u16);

#[bitfield]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct QueueId(u16);

#[bitfield]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct CommandId(u16);

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum QueuePriority {
    Urgent,
    High,
    Medium,
    Low,
}
