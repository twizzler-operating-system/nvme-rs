use modular_bitfield::prelude::*;

use crate::ds::{Microseconds, Minutes, OneHundredMilliseconds, Seconds};

#[repr(C)]
struct IdentifyControllerDataStructure {
    pub vendor: u16,
    pub subsystem_vendor_id: u16,
    pub serial_number: [u8; 20],
    pub model_number: [u8; 40],
    pub firmware_revision: u64,
    pub reccommended_arbitration_burst: u8,
    pub ieee_oui_identifier: [u8; 3],
    pub multipath_io_and_namespace_sharing_caps: MultipathIONamespaceSharingCaps,
    pub max_data_transfer_size: u8,
    pub controller_id: u16,
    pub version: u32,
    pub rtd3_resume_latency: Microseconds,
    pub rtd3_entry_latency: Microseconds,
    pub optional_async_events_supported: OptionalAsyncEventsSupported,
    pub controller_attributes: ControllerAttributes,
    pub read_recovery_levels_supported: ReadRecoveryLevelsSupported,
    pub res0: [u8; 9],
    pub controller_type: ControllerType,
    pub fru_globally_unique_identifier: u128,
    pub command_retry_delay_time_1: OneHundredMilliseconds,
    pub command_retry_delay_time_2: OneHundredMilliseconds,
    pub command_retry_delay_time_3: OneHundredMilliseconds,
    pub res1: [u8; 106],
    pub res2: [u8; 13],
    pub nvm_subsystem_report: NvmSubsystemReport,
    pub vpd_write_cycle_info: VPDWriteCycleInfo,
    pub management_endpoint_capabilities: ManagementEndpointCapabilities,
    pub optional_admin_command_support: OptionalAdminCommandSupport,
    pub abort_command_limit: u8,
    pub async_event_request_limit: u8,
    pub firmware_updates: FirmwareUpdates,
    pub log_page_attributes: LogPageAttributes,
    pub error_log_page_entries: u8,
    pub nr_power_states_support: u8,
    pub admin_vendor_specific_command_config: u8,
    pub autonomous_power_state_transition_attributes: u8,
    pub warning_composite_temp_threshold: u16,
    pub critical_composite_temp_threshold: u16,
    pub max_time_for_firmware_activation: OneHundredMilliseconds,
    pub host_memory_buffer_preferred_size: u32,
    pub host_memory_buffer_minimum_size: u32,
    pub total_nvm_capacity: u128,
    pub unallocated_nvm_capacity: u128,
    pub replay_protected_memory_block_support: ReplayProtectedMemoryBlockSupport,
    pub extended_device_self_test_time: Minutes,
    pub device_self_test_options: u8,
    pub firmware_upgrade_granularity: u8,
    pub keep_alive_support: OneHundredMilliseconds,
    pub host_controlled_thermal_management_attributes: u16,
    pub min_thermal_management_temp: u16,
    pub max_thermal_management_temp: u16,
    pub sanitize_capabilities: SanitizeCapabilities,
    pub host_memory_buffer_min_desc_entry_size: u32,
    pub host_memory_buffer_max_desc_entries: u16,
    pub nvm_set_ident_maximum: u16,
    pub endurance_group_ident_maximum: u16,
    pub ana_transition_time: Seconds,
}

const SizeChecker: [u8; 0x1000] = [0; std::mem::size_of::<IdentifyControllerDataStructure>()];

#[bitfield(bits = 8)]
pub struct MultipathIONamespaceSharingCaps {
    #[skip(setters)]
    pub nvm_sub_multiple: B1,
    #[skip(setters)]
    pub nvm_sub_two_or_more: B1,
    #[skip(setters)]
    pub sriov_controller: B1,
    #[skip(setters)]
    pub asym_namespace_access_reporting: B1,
    #[skip]
    res: B4,
}

#[bitfield(bits = 32)]
pub struct OptionalAsyncEventsSupported {
    #[skip]
    res: B8,
    #[skip(setters)]
    pub namespace_attribute_notices: B1,
    #[skip(setters)]
    pub firmware_activation_notices: B1,
    #[skip]
    res1: B1,
    #[skip(setters)]
    pub asym_namespace_access_change: B1,
    #[skip(setters)]
    pub predictable_latency_event_aggregate_log: B1,
    #[skip(setters)]
    pub lba_status_info_alert: B1,
    #[skip(setters)]
    pub endurance_group_event_aggregate_log: B1,
    #[skip(setters)]
    pub normal_nvm_subsystem_shutdown: B1,
    #[skip]
    res2: B11,
    #[skip(setters)]
    pub zone_desc_changed_notices: B1,
    #[skip]
    res3: B3,
    #[skip(setters)]
    pub discovery_log_page_change: B1,
}

#[bitfield(bits = 32)]
pub struct ControllerAttributes {
    #[skip(setters)]
    pub host_id: B1,
    #[skip(setters)]
    pub non_op_power_state_permissive_mode: B1,
    #[skip(setters)]
    pub nvm_sets: B1,
    #[skip(setters)]
    pub read_recovery_levels: B1,
    #[skip(setters)]
    pub endurance_groups: B1,
    #[skip(setters)]
    pub predictable_latency_mode: B1,
    #[skip(setters)]
    pub traffic_based_keep_alive: B1,
    #[skip(setters)]
    pub namespace_granularity: B1,
    #[skip(setters)]
    pub sq_associations: B1,
    #[skip(setters)]
    pub uuid_list: B1,
    #[skip(setters)]
    pub multi_domain_subsystem: B1,
    #[skip(setters)]
    pub fixed_domain_capacity_management: B1,
    #[skip(setters)]
    pub variable_capacity_management: B1,
    #[skip(setters)]
    pub delete_endurance_group: B1,
    #[skip(setters)]
    pub delete_nvm_set: B1,
    #[skip(setters)]
    pub extended_lba_formats_supported: B1,
    #[skip]
    res: B16,
}

pub struct ReadRecoveryLevelsSupported(u16);

#[repr(u8)]
pub enum ControllerType {
    Reserved,
    IOController,
    DiscoveryController,
    AdministrativeController,
}

#[bitfield(bits = 8)]
pub struct NvmSubsystemReport {
    #[skip(setters)]
    pub nvme_storage_device: B1,
    #[skip(setters)]
    pub nvme_enclosure: B1,
    #[skip]
    res: B6,
}

#[bitfield(bits = 8)]
pub struct VPDWriteCycleInfo {
    #[skip(setters)]
    pub write_cycles_remaining: B7,
    #[skip(setters)]
    pub valid: B1,
}

#[bitfield(bits = 8)]
pub struct ManagementEndpointCapabilities {
    #[skip]
    pub res: B6,
    #[skip(setters)]
    pub pcie_management_endpoint: B1,
    #[skip(setters)]
    pub smbus_port_management_endpoint: B1,
}

#[bitfield(bits = 16)]
pub struct OptionalAdminCommandSupport {
    #[skip(setters)]
    pub security_send_and_recv: B1,
    #[skip(setters)]
    pub format_nvm: B1,
    #[skip(setters)]
    pub firmware_download_and_commit: B1,
    #[skip(setters)]
    pub namespace_management: B1,
    #[skip(setters)]
    pub device_self_test: B1,
    #[skip(setters)]
    pub directives: B1,
    #[skip(setters)]
    pub nvme_mi_send_and_recv: B1,
    #[skip(setters)]
    pub virtualization_management: B1,
    #[skip(setters)]
    pub doorbell_buffer_config: B1,
    #[skip(setters)]
    pub get_lba_status: B1,
    #[skip(setters)]
    pub command_and_feature_lockdown: B1,
    #[skip]
    res: B5,
}

#[bitfield(bits = 8)]
pub struct FirmwareUpdates {
    #[skip(setters)]
    pub first_firmware_slot_readonly: B1,
    #[skip(setters)]
    pub number_of_firmware_slots: B3,
    #[skip(setters)]
    pub firmware_activation_without_reset: B1,
    #[skip(setters)]
    pub support_multiple_update_detection: B1,
    #[skip]
    res: B2,
}

#[bitfield(bits = 8)]
pub struct LogPageAttributes {
    #[skip(setters)]
    pub per_namespace_smart_log: B1,
    #[skip(setters)]
    pub commands_supported_and_effects: B1,
    #[skip(setters)]
    pub get_log_page: B1,
    #[skip(setters)]
    pub telemetry_host_initiated: B1,
    #[skip(setters)]
    pub persistent_event_log: B1,
    #[skip(setters)]
    pub supported_effects_features_and_nvme_mi: B1,
    #[skip(setters)]
    pub data_area_4: B1,
    #[skip]
    res: B1,
}

#[bitfield(bits = 32)]
pub struct ReplayProtectedMemoryBlockSupport {
    #[skip(setters)]
    pub nr_rpmb_units: B3,
    #[skip(setters)]
    pub auth_method: AuthMethod,
    #[skip]
    res: B10,
    #[skip(setters)]
    pub total_size: B8,
    #[skip(setters)]
    pub access_size: B8,
}

#[derive(BitfieldSpecifier)]
#[bits = 3]
pub enum AuthMethod {
    HmacSha256,
}

#[bitfield(bits = 32)]
pub struct SanitizeCapabilities {
    #[skip(setters)]
    pub crypto_erase: B1,
    #[skip(setters)]
    pub block_erase: B1,
    #[skip(setters)]
    pub overwrite: B1,
    #[skip]
    res: B26,
    #[skip(setters)]
    pub no_dealloc_inhibited: B1,
    #[skip(setters)]
    pub no_dealloc_mods_media: B2,
}
