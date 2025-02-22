use super::{queue::subentry::Dptr, Address};

pub mod admin;

pub enum PrpListOrBuffer {
    PrpList(Address),
    PrpFirstAndList(Address, Address),
    Buffer(Address),
}

impl PrpListOrBuffer {
    pub fn dptr(&self) -> Dptr {
        match self {
            PrpListOrBuffer::PrpList(address) => Dptr::Prp(*address, 0),
            PrpListOrBuffer::Buffer(address) => Dptr::Prp(*address, 0),
            PrpListOrBuffer::PrpFirstAndList(first, list) => Dptr::Prp(*first, *list),
        }
    }
}
