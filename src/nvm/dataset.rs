use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Default)]
#[bits = 2]
pub enum AccessLatency {
    #[default]
    NoInfo,
    Idle,
    Normal,
    Low,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Default)]
#[bits = 4]
pub enum AccessFrequency {
    #[default]
    NoInfo,
    Typical,
    InfrequentBoth,
    InfrequentWrites,
    InfrequentReads,
    FrequentBoth,
    OneTime,
    Speculative,
    OverwriteSoon,
}
