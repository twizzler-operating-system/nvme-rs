use modular_bitfield::prelude::*;

use crate::{
    ds::{
        cmd::admin::AdminCommand,
        queue::{
            subentry::{CommandDword0, CommonCommand, Dptr, FuseSpec, Psdt},
            CommandId, QueueId, QueuePriority, QueueSize,
        },
        Address,
    },
    host_memory::PrpListOrBuffer,
};

// REF: 2b::5.5

#[bitfield(bits = 32)]
#[repr(u32)]
struct CreateIOSubmissionQueueDword10 {
    qid: QueueId,
    qsz: QueueSize,
}

#[bitfield(bits = 32)]
#[repr(u32)]
struct CreateIOSubmissionQueueDword11 {
    phys_contiguous: bool,
    #[bits = 2]
    priority: QueuePriority,
    #[skip]
    res: B13,
    cqid: QueueId,
}

pub struct CreateIOSubmissionQueue {
    dw10: CreateIOSubmissionQueueDword10,
    dw11: CreateIOSubmissionQueueDword11,
    prp: Address,
    cdw0: CommandDword0,
}

impl CreateIOSubmissionQueue {
    pub fn new(
        cid: CommandId,
        qid: QueueId,
        prp: PrpListOrBuffer,
        qsz: QueueSize,
        cqid: QueueId,
        pri: QueuePriority,
    ) -> Self {
        Self {
            dw10: CreateIOSubmissionQueueDword10::new()
                .with_qid(qid)
                .with_qsz(qsz),
            dw11: CreateIOSubmissionQueueDword11::new()
                .with_phys_contiguous(!prp.is_list())
                .with_priority(pri)
                .with_cqid(cqid),
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

impl From<CreateIOSubmissionQueue> for CommonCommand {
    fn from(c: CreateIOSubmissionQueue) -> Self {
        Self::new()
            .with_cdw0(c.cdw0)
            .with_cdw10(c.dw10.into())
            .with_cdw11(c.dw11.into())
            .with_dptr(Dptr::Prp(c.prp, 0))
    }
}
