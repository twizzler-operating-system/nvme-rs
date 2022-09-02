use std::ops::Add;

use modular_bitfield::prelude::*;

use crate::ds::{namespace::NamespaceId, sgl::SglDescriptor, Address};

use super::CommandId;
// 2b::3.3.3.1

#[bitfield(bits = 32)]
#[derive(Default, Clone, Copy, Debug)]
pub struct CommandDword0 {
    op: B8,
    fuse: FuseSpec,
    #[skip]
    res0: B4,
    psdt: Psdt,
    cid: CommandId,
}

impl CommandDword0 {
    pub fn build(op: u8, cid: CommandId, fuse: FuseSpec, psdt: Psdt) -> Self {
        Self::new()
            .with_op(op)
            .with_cid(cid)
            .with_fuse(fuse)
            .with_psdt(psdt)
    }
}

#[derive(BitfieldSpecifier, Clone, Copy, Debug)]
#[bits = 2]
pub enum Psdt {
    Prp,
    Sgl,
    SglAndMeta,
}

#[derive(BitfieldSpecifier, Clone, Copy, Debug)]
#[bits = 2]
pub enum FuseSpec {
    Normal,
    FuseFirst,
    FuseSecond,
}

#[derive(Clone, Copy)]
#[repr(C)]
union DptrData {
    prp: [Address; 2],
    sgl: SglDescriptor,
}

impl Default for DptrData {
    fn default() -> Self {
        Self { prp: [0; 2] }
    }
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct CommonCommand {
    cdw0: CommandDword0,
    nsid: NamespaceId,
    cdw2: u32,
    cdw3: u32,
    mptr: Address,
    dptr: DptrData,
    cdw10: u32,
    cdw11: u32,
    cdw12: u32,
    cdw13: u32,
    cdw14: u32,
    cdw15: u32,
}

impl CommonCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cdw0(self, cdw0: CommandDword0) -> Self {
        Self { cdw0, ..self }
    }

    pub fn with_nsid(self, nsid: NamespaceId) -> Self {
        Self { nsid, ..self }
    }

    pub fn with_cdw2(self, cdw2: u32) -> Self {
        Self { cdw2, ..self }
    }

    pub fn with_cdw3(self, cdw3: u32) -> Self {
        Self { cdw3, ..self }
    }

    pub fn with_cdw10(self, cdw10: u32) -> Self {
        Self { cdw10, ..self }
    }

    pub fn with_cdw11(self, cdw11: u32) -> Self {
        Self { cdw11, ..self }
    }

    pub fn with_cdw12(self, cdw12: u32) -> Self {
        Self { cdw12, ..self }
    }

    pub fn with_cdw13(self, cdw13: u32) -> Self {
        Self { cdw13, ..self }
    }

    pub fn with_cdw14(self, cdw14: u32) -> Self {
        Self { cdw14, ..self }
    }

    pub fn with_cdw15(self, cdw15: u32) -> Self {
        Self { cdw15, ..self }
    }

    pub fn with_mptr(self, mptr: Address) -> Self {
        Self { mptr, ..self }
    }

    pub fn with_dptr(self, dptr: Dptr) -> Self {
        match dptr {
            Dptr::Prp(a1, a2) => Self {
                dptr: DptrData { prp: [a1, a2] },
                ..self
            },
            Dptr::Sgl(s) => Self {
                dptr: DptrData { sgl: s },
                ..self
            },
        }
    }

    pub fn with_cid(self, cid: CommandId) -> Self {
        let n = self.cdw0.with_cid(cid);
        self.with_cdw0(n)
    }

    pub fn set_cid(&mut self, cid: CommandId) {
        self.cdw0.set_cid(cid);
    }
}

pub enum Dptr {
    Prp(Address, Address),
    Sgl(SglDescriptor),
}

impl Dptr {
    pub fn psdt(&self, meta_is_sgl: bool) -> Psdt {
        match self {
            Dptr::Prp(_, _) => Psdt::Prp,
            Dptr::Sgl(_) => {
                if meta_is_sgl {
                    Psdt::SglAndMeta
                } else {
                    Psdt::Sgl
                }
            }
        }
    }
}
