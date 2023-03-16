mod dataset;
mod read;
mod write;

pub use dataset::{
    AccessFrequency, AccessLatency, ContextAttributes, DatasetMgmtCommand, DatasetMgmtDword11,
};
pub use read::{ReadCommand, ReadDword13};
pub use write::{WriteCommand, WriteDword13};

#[repr(u8)]
pub enum NvmCommand {
    Write = 1,
    Read = 2,
    DatasetMgmt = 9,
}

impl From<NvmCommand> for u8 {
    fn from(a: NvmCommand) -> Self {
        a as u8
    }
}
