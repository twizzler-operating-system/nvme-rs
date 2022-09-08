use modular_bitfield::bitfield;

pub mod comentry;
pub mod subentry;

use modular_bitfield::prelude::*;

#[bitfield]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct QueueSize(u16);

#[bitfield]
#[derive(BitfieldSpecifier, Clone, Copy)]
#[repr(transparent)]
pub struct QueueId(u16);

impl QueueId {
    pub const ADMIN: Self = Self::new();
}

impl From<QueueId> for usize {
    fn from(qi: QueueId) -> Self {
        qi.get_0().into()
    }
}

#[bitfield]
#[derive(BitfieldSpecifier, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct CommandId(u16);

impl From<u16> for CommandId {
    fn from(x: u16) -> Self {
        CommandId::new().with_0(x)
    }
}

impl From<CommandId> for u16 {
    fn from(x: CommandId) -> Self {
        x.get_0()
    }
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum QueuePriority {
    Urgent,
    High,
    Medium,
    Low,
}
