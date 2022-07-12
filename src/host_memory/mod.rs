use crate::ds::Address;

pub struct VirtualRegion {}

pub enum PrpListOrBuffer {
    PrpList(Address),
    PrpBuffer(Address),
}

impl PrpListOrBuffer {
    pub fn address(&self) -> Address {
        match self {
            PrpListOrBuffer::PrpList(a) => *a,
            PrpListOrBuffer::PrpBuffer(a) => *a,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            PrpListOrBuffer::PrpList(_) => true,
            PrpListOrBuffer::PrpBuffer(_) => false,
        }
    }
}

impl VirtualRegion {
    pub fn base<T>(&self) -> *const T {
        todo!()
    }

    pub fn base_mut<T>(&mut self) -> *mut T {
        todo!()
    }
}
