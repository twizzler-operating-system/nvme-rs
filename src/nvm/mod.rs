mod dataset;
mod read;

pub use dataset::{AccessFrequency, AccessLatency};
pub use read::{ReadCommand, ReadDword13};

#[repr(u8)]
pub enum NvmCommand {
    Read = 2,
}

impl From<NvmCommand> for u8 {
    fn from(a: NvmCommand) -> Self {
        a as u8
    }
}
