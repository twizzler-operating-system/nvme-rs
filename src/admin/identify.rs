use crate::ds::{
    controller::ControllerId,
    queue::subentry::{CommandDword0, CommonCommand, Dptr},
};

use modular_bitfield::prelude::*;
#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword10 {
    cntid: ControllerId,
    #[skip]
    res: B8,
    cns: B8,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword11 {
    cns_specific_id: u16,
    #[skip]
    res: B8,
    csi: u8,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct IdentifyDword14 {
    uuid_idx: B7,
    #[skip]
    res: B25,
}

struct Identify {
    dw10: IdentifyDword10,
    dw11: IdentifyDword11,
    dw14: IdentifyDword14,
    cdw0: CommandDword0,
    dptr: Dptr,
}

impl From<Identify> for CommonCommand {
    fn from(i: Identify) -> Self {
        Self::new()
            .with_cdw0(i.cdw0)
            .with_cdw10(i.dw10.into())
            .with_cdw11(i.dw11.into())
            .with_cdw14(i.dw14.into())
            .with_dptr(i.dptr)
    }
}
