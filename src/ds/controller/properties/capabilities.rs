use modular_bitfield::prelude::*;

use crate::ds::HalfSeconds;

#[derive(Clone, Copy)]
#[bitfield(bits = 64)]
#[repr(u64)]
pub struct ControllerCap {
    #[skip(setters)]
    pub max_queue_entries: u16,
    #[skip(setters)]
    pub contiguous_queues_required: bool,
    #[skip(setters)]
    arbitration_mechanism: B2,
    #[skip]
    res: B5,
    #[skip(setters)]
    pub timeout: HalfSeconds,
    #[skip(setters)]
    pub doorbell_stride: B4,
    #[skip(setters)]
    pub nvm_subsystem_reset_supported: bool,
    #[skip(setters)]
    command_sets_supported: B8,
    #[skip(setters)]
    pub boot_partition_supported: bool,
    #[skip(setters)]
    pub controller_power_scope: ControllerPowerScope,
    #[skip(setters)]
    pub memory_page_sz_min: B4,
    #[skip(setters)]
    pub memory_page_sz_max: B4,
    #[skip(setters)]
    pub persistent_mem_region_supported: bool,
    #[skip(setters)]
    pub controller_mem_buffer_supported: bool,
    #[skip(setters)]
    pub nvm_subsystem_shutdown_supported: bool,
    #[skip(setters)]
    pub controller_ready_modes_supported: B2,
    #[skip]
    res2: B3,
}

#[derive(Clone, Copy, BitfieldSpecifier)]
#[bits = 2]
pub enum ControllerPowerScope {
    NotReported,
    Controller,
    Domain,
    NVMSubSystem,
}

impl ControllerCap {
    pub fn controller_ready_with_media_support(&self) -> bool {
        self.controller_ready_modes_supported() & 1 != 0
    }

    pub fn controller_ready_independent_of_media_support(&self) -> bool {
        self.controller_ready_modes_supported() & 2 != 0
    }

    pub fn arbitration_urgent_with_wrr(&self) -> bool {
        self.arbitration_mechanism() & 1 != 0
    }

    pub fn arbitration_vendor_specific(&self) -> bool {
        self.arbitration_mechanism() & 2 != 0
    }

    pub fn supports_nvm_command_set(&self) -> bool {
        self.command_sets_supported() & 1 != 0
    }

    pub fn supports_more_io_command_sets(&self) -> bool {
        self.command_sets_supported() & (1 << 6) != 0
    }

    pub fn doorbell_stride_bytes(&self) -> usize {
        1 << (self.doorbell_stride() as usize + 2)
    }

    pub fn memory_page_sz_min_bytes(&self) -> usize {
        1 << (self.memory_page_sz_min() + 12)
    }

    pub fn memory_page_sz_max_bytes(&self) -> usize {
        1 << (self.memory_page_sz_max() + 12)
    }
}
