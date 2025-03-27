#![allow(dead_code)]
use modular_bitfield::prelude::*;

use super::{
    dataset::{AccessFrequency, AccessLatency},
    NvmCommand,
};
use crate::ds::{
    namespace::NamespaceId,
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec, Psdt},
        CommandId,
    },
};

#[bitfield(bits = 32)]
#[repr(u32)]
struct WriteDword10 {
    start_lba_lo: u32,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct WriteDword11 {
    start_lba_hi: u32,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct WriteDword12 {
    pub nr_blocks: u16,
    #[skip]
    resv: B14,
    pub force_unit_access: B1,
    pub limited_retry: B1,
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub struct WriteDword13 {
    pub access_frequency: AccessFrequency,
    pub access_latency: AccessLatency,
    pub sequenial: bool,
    pub incompressable: bool,
    #[skip]
    resv: B24,
}

pub struct WriteCommand {
    dw10: WriteDword10,
    dw11: WriteDword11,
    dw12: WriteDword12,
    dw13: WriteDword13,
    dptr: Dptr,
    cdw0: CommandDword0,
    nsid: NamespaceId,
}

impl WriteCommand {
    /// Construct a Create IO Completion Queue request. See base spec section 5.4 for more details.
    pub fn new(
        cid: CommandId,
        nsid: NamespaceId,
        dptr: Dptr,
        start_lba: u64,
        nr_blocks: u16,
        access_info: WriteDword13,
    ) -> Self {
        Self {
            dw10: WriteDword10::new().with_start_lba_lo(start_lba as u32),
            dw11: WriteDword11::new().with_start_lba_hi((start_lba >> 32) as u32),
            dw12: WriteDword12::new().with_nr_blocks(nr_blocks - 1),
            dw13: access_info,
            dptr,
            cdw0: CommandDword0::build(NvmCommand::Write.into(), cid, FuseSpec::Normal, Psdt::Prp),
            nsid,
        }
    }
}

impl From<WriteCommand> for CommonCommand {
    fn from(c: WriteCommand) -> Self {
        Self::new()
            .with_cdw0(c.cdw0)
            .with_cdw10(c.dw10.into())
            .with_cdw11(c.dw11.into())
            .with_cdw12(c.dw12.into())
            .with_cdw13(c.dw13.into())
            .with_nsid(c.nsid)
            .with_dptr(c.dptr)
    }
}
