use core::{cmp::min, fmt::Debug};

use modular_bitfield::prelude::*;

#[derive(Clone)]
#[repr(C)]
pub struct IdentifyNamespaceDataStructure {
    pub size: u64,
    pub capacity: u64,
    pub utilization: u64,
    pub features: NamespaceFeatures,
    num_lba_formats: u8,
    pub formatted_lba_size: FormattedLbaSize,
    _resv: [u8; 101],
    lba_format_support: [LbaFormat; 64],
    _resv2: [u8; 4096 - 384],
}

impl Default for IdentifyNamespaceDataStructure {
    fn default() -> Self {
        Self {
            size: Default::default(),
            capacity: Default::default(),
            utilization: Default::default(),
            features: Default::default(),
            num_lba_formats: Default::default(),
            formatted_lba_size: Default::default(),
            _resv: [0; 101],
            lba_format_support: [Default::default(); 64],
            _resv2: [0; 4096 - 384],
        }
    }
}

impl Debug for IdentifyNamespaceDataStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentifyNamespaceDataStructure")
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .field("utilization", &self.utilization)
            .field("features", &self.features)
            .field("num_lba_formats", &self.num_lba_formats)
            .field("formatted_lba_size", &self.formatted_lba_size)
            .field("lba_format_support", &self.lba_formats())
            .finish()
    }
}

impl IdentifyNamespaceDataStructure {
    pub fn lba_formats(&self) -> &[LbaFormat] {
        &self.lba_format_support[0..=min(self.num_lba_formats.into(), 63)]
    }
}

const _SIZE_CHECKER: [u8; 0x1000] = [0; std::mem::size_of::<IdentifyNamespaceDataStructure>()];

#[bitfield(bits = 32)]
#[derive(Default, Clone, Debug, Copy)]
pub struct LbaFormat {
    #[skip(setters)]
    pub metadata_size: u16,
    #[skip(setters)]
    pub data_size_log2: u8,
    #[skip(setters)]
    pub relative_performance: RelativePerformance,
    #[skip]
    res: B6,
}

impl LbaFormat {
    pub fn data_size(&self) -> usize {
        1 << self.data_size_log2()
    }
}

#[derive(BitfieldSpecifier, Clone, Debug)]
#[bits = 2]
pub enum RelativePerformance {
    Best = 0,
    Better = 1,
    Good = 2,
    Degraded = 3,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone)]
pub struct FormattedLbaSize {
    #[skip(setters)]
    idx_lo: B4,
    #[skip(setters)]
    pub extended_block: B1,
    #[skip(setters)]
    idx_hi: B2,
    #[skip]
    resv: B1,
}

impl FormattedLbaSize {
    pub fn index(&self) -> usize {
        usize::from(self.idx_lo()) | usize::from(self.idx_hi()) << 4
    }
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone)]
pub struct NamespaceFeatures {
    #[skip(setters)]
    pub supports_thin_provisioning: B1,
    #[skip(setters)]
    pub supports_atomic_fields: B1,
    #[skip(setters)]
    pub supports_dealloc_lb_err: B1,
    #[skip(setters)]
    pub uid_reuse: B1,
    #[skip(setters)]
    pub opt_perf: B1,
    #[skip]
    resv: B3,
}
