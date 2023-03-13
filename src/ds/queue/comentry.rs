use modular_bitfield::prelude::*;

use super::{CommandId, QueueId};

// 2b::3.3.3.2

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CommonCompletion {
    dw0: u32,
    dw1: u32,
    sqinfo: SqInfo,
    status: CompletionStatus,
}

impl Default for CommonCompletion {
    fn default() -> Self {
        Self {
            dw0: Default::default(),
            dw1: Default::default(),
            sqinfo: SqInfo::new(),
            status: CompletionStatus::new(),
        }
    }
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy)]
struct CompletionStatus {
    cid: B16,
    phase: bool,
    status: StatusField,
}

#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy)]
struct SqInfo {
    head: B16,
    sqid: B16,
}

#[bitfield(bits = 15)]
#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
pub struct StatusField {
    code: B8,
    code_type: B3,
    retry_delay: B2,
    more: B1,
    do_not_retry: B1,
}

impl StatusField {
    pub fn is_error(&self) -> bool {
        self.code() != 0
    }
}

impl CommonCompletion {
    pub fn phase(&self) -> bool {
        self.status.phase()
    }

    pub fn new_sq_head(&self) -> u16 {
        self.sqinfo.head()
    }

    pub fn sq_id(&self) -> QueueId {
        QueueId::new().with_0(self.sqinfo.sqid())
    }

    pub fn status(&self) -> StatusField {
        self.status.status()
    }

    pub fn command_id(&self) -> CommandId {
        CommandId::new().with_0(self.status.cid())
    }
}
