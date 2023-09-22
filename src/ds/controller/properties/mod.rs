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
    pub capabilities: ControllerCap,
    pub version: u32,
    pub int_mask_set: u32,
    pub int_mask_clear: u32,
    pub configuration: ControllerConfig,
    resv: u32,
    pub status: ControllerStatus,
    pub nvm_subsystem_reset: u32,
    pub admin_queue_attr: AdminQueueAttributes,
    pub admin_subqueue_base_addr: u64,
    pub admin_comqueue_base_addr: u64,
    pub memory_buffer_location: u32,
    pub memory_buffer_size: u32,
    pub boot_partition_info: u32,
    pub boot_partition_read_select: u32,
    pub boot_partition_memory_buffer_location: u64,
    pub memory_buffer_memory_space_control: u64,
    pub memory_buffer_status: u32,
    pub memory_buffer_elasticity_buffer_size: u32,
    pub memory_buffer_sustained_write_throughput: u32,
    pub nvm_subsystem_shutdown: u32,
    pub controller_ready_timeouts: u32,
    resv2: u32,
    resv3: [u8; 0xD90], //TODO: verify
    pub pmem_capabilities: u32,
    pub pmem_region_control: u32,
    pub pmem_region_status: u32,
    pub pmem_region_elasticity_buffer_size: u32,
    pub pmem_region_sustained_write_throughput: u32,
    pub pmem_region_controller_mem_space_control_lower: u32,
    pub pmem_region_controller_mem_space_control_upper: u32,
}

impl ControllerProperties {
    pub fn version_maj(&self) -> u16 {
        (self.version >> 16) as u16
    }

    pub fn version_min(&self) -> u8 {
        ((self.version >> 8) & 0xff) as u8
    }
}
