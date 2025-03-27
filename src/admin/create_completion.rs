#![allow(dead_code)]
use modular_bitfield::prelude::*;

use crate::ds::{
    cmd::{admin::AdminCommand, PrpListOrBuffer},
    queue::{
        subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec, Psdt},
        CommandId, QueueId, QueueSize,
    },
    Address, InterruptVector,
};

// REF: 2b::5.4

#[bitfield(bits = 32)]
#[repr(u32)]
struct CreateIOCompletionQueueDword10 {
    #[allow(dead_code)]
    qid: QueueId,
    #[allow(dead_code)]
    qsz: QueueSize,
}

#[allow(dead_code)]
#[bitfield(bits = 32)]
#[repr(u32)]
struct CreateIOCompletionQueueDword11 {
    phys_contiguous: bool,
    int_enabled: bool,
    #[skip]
    res: B14,
    ivec: InterruptVector,
}

pub struct CreateIOCompletionQueue {
    dw10: CreateIOCompletionQueueDword10,
    dw11: CreateIOCompletionQueueDword11,
    prp: Address,
    cdw0: CommandDword0,
}

impl CreateIOCompletionQueue {
    /// Construct a Create IO Completion Queue request. See base spec section 5.4 for more details.
    pub fn new(
        cid: CommandId,
        qid: QueueId,
        prp: PrpListOrBuffer,
        qsz: QueueSize,
        ivec: InterruptVector,
        ien: bool,
    ) -> Self {
        Self {
            dw10: CreateIOCompletionQueueDword10::new()
                .with_qid(qid)
                .with_qsz(qsz),
            dw11: CreateIOCompletionQueueDword11::new()
                .with_phys_contiguous(!prp.is_list())
                .with_int_enabled(ien)
                .with_ivec(ivec),
            prp: prp.address(),
            cdw0: CommandDword0::build(
                AdminCommand::CreateCompletionQueue.into(),
                cid,
                FuseSpec::Normal,
                Psdt::Prp,
            ),
        }
    }
}

impl From<CreateIOCompletionQueue> for CommonCommand {
    fn from(c: CreateIOCompletionQueue) -> Self {
        Self::new()
            .with_cdw0(c.cdw0)
            .with_cdw10(c.dw10.into())
            .with_cdw11(c.dw11.into())
            .with_dptr(Dptr::Prp(c.prp, 0))
    }
}
