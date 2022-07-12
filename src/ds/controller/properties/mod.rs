use volatile_cell::VolatileCell;

use self::{
    aqa::AdminQueueAttributes, capabilities::ControllerCap, config::ControllerConfig,
    status::ControllerStatus,
};

pub mod aqa;
pub mod capabilities;
pub mod config;
pub mod status;

#[repr(C)]
pub struct ControllerProperties {
    pub capabilities: VolatileCell<ControllerCap>,
    pub version: VolatileCell<u32>,
    pub int_mask_set: VolatileCell<u32>,
    pub int_mask_clear: VolatileCell<u32>,
    pub configuration: VolatileCell<ControllerConfig>,
    resv: u32,
    pub status: VolatileCell<ControllerStatus>,
    pub nvm_subsystem_reset: VolatileCell<u32>,
    pub admin_queue_attr: VolatileCell<AdminQueueAttributes>,
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

impl ControllerProperties {
    pub fn version_maj(&self) -> u16 {
        (self.version.get() >> 16) as u16
    }

    pub fn version_min(&self) -> u8 {
        ((self.version.get() >> 8) & 0xff) as u8
    }
}
