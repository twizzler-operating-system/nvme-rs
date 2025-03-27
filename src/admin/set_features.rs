#![allow(dead_code)]

use modular_bitfield::prelude::*;

use crate::ds::{
    cmd::admin::{features::FeatureId, AdminCommand},
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec},
        CommandId,
    },
    uuid::UuidIndex,
};

// REF: 2b::5.5

#[bitfield(bits = 32)]
#[repr(u32)]
struct SetFeaturesDword10 {
    fid: FeatureId,
    #[skip]
    res: B23,
    save: bool,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct SetFeaturesDword14 {
    uuid_index: UuidIndex,
    #[skip]
    res: B25,
}

pub struct SetFeatures {
    dw10: SetFeaturesDword10,
    dw14: SetFeaturesDword14,
    dptr: Dptr,
    cdw0: CommandDword0,
}

impl SetFeatures {
    pub fn new(
        cid: CommandId,
        fid: FeatureId,
        save: bool,
        dptr: Dptr,
        uuid_index: Option<UuidIndex>,
    ) -> Self {
        Self {
            dw10: SetFeaturesDword10::new().with_save(save).with_fid(fid),
            dw14: SetFeaturesDword14::new().with_uuid_index(uuid_index.into()),
            cdw0: CommandDword0::build(
                AdminCommand::CreateCompletionQueue.into(),
                cid,
                FuseSpec::Normal,
                dptr.psdt(false),
            ),
            dptr,
        }
    }
}

impl From<SetFeatures> for CommonCommand {
    fn from(sf: SetFeatures) -> Self {
        Self::new()
            .with_cdw0(sf.cdw0)
            .with_cdw10(sf.dw10.into())
            .with_cdw14(sf.dw14.into())
            .with_dptr(sf.dptr)
    }
}
