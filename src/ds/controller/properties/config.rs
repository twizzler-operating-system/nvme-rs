use modular_bitfield::prelude::*;

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct ControllerConfig {
    pub enable: bool,
    #[skip]
    res: B3,
    pub io_command_set_selected: IOCommandSet,
    pub mem_page_size: B4,
    pub arbitration_mechanism: ArbitrationMechanism,
    pub shutdown_notification: ShutdownNotification,
    pub io_submission_queue_entry_size: B4,
    pub io_completion_queue_entry_size: B4,
    pub controller_ready_independent_of_media_enable: bool,
    #[skip]
    res1: B7,
}

#[derive(BitfieldSpecifier)]
#[bits = 3]
pub enum IOCommandSet {
    NVMCommandSet,
    AllSupported = 0b110,
    AdminOnly = 0b111,
}

#[derive(BitfieldSpecifier)]
#[bits = 3]
pub enum ArbitrationMechanism {
    RoundRobin,
    WeightedRoundRobinWithUrgent,
    VendorSpecific = 0b111,
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum ShutdownNotification {
    NoNotification,
    NormalShutdown,
    AbruptShutdown,
}
