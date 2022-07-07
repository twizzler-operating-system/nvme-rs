use modular_bitfield::prelude::*;

// 2b::3.3.3.2

#[repr(C)]
struct CommonCompletion {
    dw0: u32,
    dw1: u32,
    sqinfo: SqInfo,
    status: CompletionStatus,
}

#[bitfield(bits = 32)]
struct CompletionStatus {
    cid: B16,
    phase: B1,
    status: StatusField,
}

#[bitfield(bits = 32)]
struct SqInfo {
    head: B16,
    sqid: B16,
}

#[bitfield(bits = 15)]
#[derive(BitfieldSpecifier)]
struct StatusField {
    code: B8,
    code_type: B3,
    retry_delay: B2,
    more: B1,
    do_not_retry: B1,
}
