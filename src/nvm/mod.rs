mod dataset;
mod read;

#[repr(u8)]
pub enum NvmCommand {
    Read = 2,
}

impl From<NvmCommand> for u8 {
    fn from(a: NvmCommand) -> Self {
        a as u8
    }
}
