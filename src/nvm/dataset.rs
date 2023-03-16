use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Default)]
#[bits = 2]
pub enum AccessLatency {
    #[default]
    NoInfo,
    Idle,
    Normal,
    Low,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Default)]
#[bits = 4]
pub enum AccessFrequency {
    #[default]
    NoInfo,
    Typical,
    InfrequentBoth,
    InfrequentWrites,
    InfrequentReads,
    FrequentBoth,
    OneTime,
    Speculative,
    OverwriteSoon,
}

use crate::ds::{
    namespace::NamespaceId,
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec, Psdt},
        CommandId,
    },
};

use super::NvmCommand;

#[bitfield(bits = 32)]
#[repr(u32)]
struct DatasetMgmtDword10 {
    nr_ranges_z: u8,
    #[skip]
    resv: B24,
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct DatasetMgmtDword11 {
    integral_for_read: bool,
    integral_for_write: bool,
    deallocate: bool,
    #[skip]
    resv: B29,
}

pub struct DatasetMgmtCommand {
    dw10: DatasetMgmtDword10,
    dw11: DatasetMgmtDword11,
    dptr: Dptr,
    cdw0: CommandDword0,
    nsid: NamespaceId,
}

#[bitfield(bits = 32)]
pub struct ContextAttributes {
    pub access_frequency: AccessFrequency,
    pub access_latency: AccessLatency,
    #[skip]
    resv: B2,
    pub seq_read_range: bool,
    pub seq_write_range: bool,
    pub write_prepare: bool,
    #[skip]
    resv2: B13,
    pub command_access_size: u8,
}

impl DatasetMgmtCommand {
    /// Construct a Create IO Completion Queue request. See base spec section 5.4 for more details.
    pub fn new(
        cid: CommandId,
        nsid: NamespaceId,
        dptr: Dptr,
        nr_ranges: u8,
        attributes: DatasetMgmtDword11,
    ) -> Self {
        Self {
            dw10: DatasetMgmtDword10::new().with_nr_ranges_z(nr_ranges - 1),
            dw11: attributes,
            dptr,
            cdw0: CommandDword0::build(
                NvmCommand::DatasetMgmt.into(),
                cid,
                FuseSpec::Normal,
                Psdt::Prp,
            ),
            nsid,
        }
    }
}

impl From<DatasetMgmtCommand> for CommonCommand {
    fn from(c: DatasetMgmtCommand) -> Self {
        Self::new()
            .with_cdw0(c.cdw0)
            .with_cdw10(c.dw10.into())
            .with_cdw11(c.dw11.into())
            .with_nsid(c.nsid)
            .with_dptr(c.dptr)
    }
}
