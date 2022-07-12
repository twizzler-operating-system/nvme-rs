use modular_bitfield::prelude::*;

#[bitfield(bits = 16)]
#[derive(BitfieldSpecifier)]
#[repr(transparent)]
pub struct ControllerId(u16);

impl From<u16> for ControllerId {
    fn from(x: u16) -> Self {
        ControllerId::new().with_0(x)
    }
}

use volatile_cell::VolatileCell;
#[repr(C)]
struct ControllerProperties {
    pub capabilities: VolatileCell<ControllerCap>,
    pub version: VolatileCell<u32>,
    pub int_mask_set: VolatileCell<u32>,
    pub int_mask_clear: VolatileCell<u32>,
    pub configuration: VolatileCell<ControllerConfig>,
    resv: u32,
    pub status: VolatileCell<u32>,
    pub nvm_subsystem_reset: VolatileCell<u32>,
    pub admin_queue_attr: VolatileCell<u32>,
    pub admin_subqueue_base_addr: VolatileCell<u64>,
    pub admin_comqueue_base_addr: VolatileCell<u64>,
    pub memory_buffer_location: VolatileCell<u32>,
    pub memory_buffer_size: VolatileCell<u32>,
    pub boot_partition_info: VolatileCell<u32>,
    pub boot_partition_read_select: VolatileCell<u32>,
    pub boot_partition_memory_buffer_location: VolatileCell<u64>,
    pub memory_buffer_memory_space_control: VolatileCell<u64>,
    pub memory_buffer_status: VolatileCell<u32>,
    pub memory_buffer_elasticity_buffer_size: VolatileCell<u32>,
    pub memory_buffer_sustained_write_throughput: VolatileCell<u32>,
    pub nvm_subsystem_shutdown: VolatileCell<u32>,
    pub controller_ready_timeouts: VolatileCell<u32>,
    resv2: u32,
    resv3: [u8; 0xD90], //TODO: verify
    pub pmem_capabilities: VolatileCell<u32>,
    pub pmem_region_control: VolatileCell<u32>,
    pub pmem_region_status: VolatileCell<u32>,
    pub pmem_region_elasticity_buffer_size: VolatileCell<u32>,
    pub pmem_region_sustained_write_throughput: VolatileCell<u32>,
    pub pmem_region_controller_mem_space_control_lower: VolatileCell<u32>,
    pub pmem_region_controller_mem_space_control_upper: VolatileCell<u32>,
}

use modular_bitfield::prelude::*;

use super::HalfSeconds;

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

#[derive(BitfieldSpecifier)]
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

impl ControllerProperties {
    pub fn version_maj(&self) -> u16 {
        (self.version.get() >> 16) as u16
    }

    pub fn version_min(&self) -> u8 {
        ((self.version.get() >> 8) & 0xff) as u8
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct ControllerConfig {
    pub enable: bool,
    res: B3,
    pub io_command_set_selected: IOCommandSet,
    pub mem_page_size: B4,
    pub arbitration_mechanism: ArbitrationMechanism,
    pub shutdown_notification: ShutdownNotification,
    pub io_submission_queue_entry_size: B4,
    pub io_completion_queue_entry_size: B4,
    pub controller_ready_independent_of_media_enable: bool,
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
