use modular_bitfield::prelude::*;

use crate::ds::{
    cmd::{admin::AdminCommand, PrpListOrBuffer},
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec, Psdt},
        CommandId, QueueId, QueueSize,
    },
    Address, InterruptVector,
};

use super::{
    dataset::{AccessFrequency, AccessLatency},
    NvmCommand,
};

#[bitfield(bits = 32)]
#[repr(u32)]
struct ReadDword10 {
    start_lba_lo: u32,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct ReadDword11 {
    start_lba_hi: u32,
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub struct ReadDword13 {
    pub access_frequency: AccessFrequency,
    pub access_latency: AccessLatency,
    pub sequenial: bool,
    pub incompressable: bool,
    #[skip]
    resv: B24,
}

pub struct CreateIOCompletionQueue {
    dw10: ReadDword10,
    dw11: ReadDword11,
    dw13: ReadDword13,
    dptr: Dptr,
    cdw0: CommandDword0,
}

impl CreateIOCompletionQueue {
    /// Construct a Create IO Completion Queue request. See base spec section 5.4 for more details.
    pub fn new(cid: CommandId, dptr: Dptr, start_lba: u64, access_info: ReadDword13) -> Self {
        Self {
            dw10: ReadDword10::new().with_start_lba_lo(start_lba as u32),
            dw11: ReadDword11::new().with_start_lba_hi((start_lba >> 32) as u32),
            dw13: access_info,
            dptr,
            cdw0: CommandDword0::build(NvmCommand::Read.into(), cid, FuseSpec::Normal, Psdt::Prp),
        }
    }
}

impl From<CreateIOCompletionQueue> for CommonCommand {
    fn from(c: CreateIOCompletionQueue) -> Self {
        Self::new()
            .with_cdw0(c.cdw0)
            .with_cdw10(c.dw10.into())
            .with_cdw11(c.dw11.into())
            .with_cdw13(c.dw13.into())
            .with_dptr(c.dptr)
    }
}
