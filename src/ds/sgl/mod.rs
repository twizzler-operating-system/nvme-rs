use modular_bitfield::prelude::*;

use super::Address;
#[bitfield(bits = 128)]
#[derive(Debug, Clone, Copy)]
pub struct SglDescriptor {
    data0: B120,
    ident: SglIdentifier,
}

#[bitfield(bits = 8)]
#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
struct SglIdentifier {
    sub_type: B4,
    desc_type: SglDescType,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy)]
#[bits = 4]
enum SglDescType {
    DataBlock,
    BitBucket,
    Segment,
    LastSegment,
    KeyedDataBlock,
    TransportDataBlock,
    VendorSpecific = 0xf,
}

#[derive(BitfieldSpecifier)]
#[bits = 4]
enum SglSubType {
    Address,
    Offset,
    TransportSpecific0 = 0xa,
    TransportSpecific1 = 0xb,
    TransportSpecific2 = 0xc,
    TransportSpecific3 = 0xd,
    TransportSpecific4 = 0xe,
    TransportSpecific5 = 0xf,
}

#[bitfield]
struct SglDataBlock {
    addr: Address,
    len: u32,
    #[skip]
    res: B24,
    id: SglIdentifier,
}

impl From<SglDataBlock> for SglDescriptor {
    fn from(s: SglDataBlock) -> Self {
        Self::from_bytes(s.into_bytes())
    }
}

#[bitfield]
struct SglBitBucket {
    #[skip]
    res0: u64,
    len: u32,
    #[skip]
    res1: B24,
    id: SglIdentifier,
}

impl From<SglBitBucket> for SglDescriptor {
    fn from(s: SglBitBucket) -> Self {
        Self::from_bytes(s.into_bytes())
    }
}

#[bitfield]
struct SglSegment {
    addr: Address,
    len: u32,
    #[skip]
    res: B24,
    id: SglIdentifier,
}

impl From<SglSegment> for SglDescriptor {
    fn from(s: SglSegment) -> Self {
        Self::from_bytes(s.into_bytes())
    }
}

#[bitfield]
struct SglKeyedDataBlock {
    addr: Address,
    len: B24,
    key: u32,
    id: SglIdentifier,
}

impl From<SglKeyedDataBlock> for SglDescriptor {
    fn from(s: SglKeyedDataBlock) -> Self {
        Self::from_bytes(s.into_bytes())
    }
}

#[bitfield]
struct SglTransportDataBlock {
    #[skip]
    res0: u64,
    len: u32,
    #[skip]
    res1: B24,
    id: SglIdentifier,
}

impl From<SglTransportDataBlock> for SglDescriptor {
    fn from(s: SglTransportDataBlock) -> Self {
        Self::from_bytes(s.into_bytes())
    }
}
