use modular_bitfield::prelude::*;

#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[repr(u32)]
pub struct ControllerStatus {
    #[skip(setters)]
    pub ready: bool,
    #[skip(setters)]
    pub fatal_status: bool,
    #[skip(setters)]
    pub shutdown_status: ShutdownStatus,
    #[skip(setters)]
    pub nvm_subsystem_reset_occurred: bool,
    #[skip(setters)]
    pub processing_paused: bool,
    #[skip(setters)]
    pub shutdown_type_is_nvm_subsystem: bool,
    #[skip]
    _res: B25,
}

#[derive(Clone, Copy, BitfieldSpecifier)]
#[bits = 2]
pub enum ShutdownStatus {
    NormalOperation,
    ShutdownProcessingOccurring,
    ShutdownProcessingComplete,
}
