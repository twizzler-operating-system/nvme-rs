use modular_bitfield::prelude::*;

#[bitfield(bits = 32)]
pub struct AdminQueueAttributes {
    pub submission_queue_size: B12,
    #[skip]
    res: B4,
    pub completion_queue_size: B12,
    #[skip]
    res2: B4,
}
