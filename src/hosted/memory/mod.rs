use crate::ds::{cmd::PrpListOrBuffer, queue::subentry::Dptr, Address};

pub struct VirtualRegion<P: PhysicalPageCollection> {
    virt: *mut u8,
    _cache_type: CacheType,
    phys_page_list: P,
    len: usize,
}

pub trait PhysicalPageCollection {
    type DmaType;
    fn get_prp_list_or_buffer(&mut self, dma: Self::DmaType) -> Option<PrpListOrBuffer>;
    fn get_dptr(&mut self, sgl_allowed: bool) -> Option<Dptr>;
}

pub enum CacheType {
    WriteBack,
    WriteThrough,
    Uncacheable,
}

impl PrpListOrBuffer {
    pub fn address(&self) -> Address {
        match self {
            PrpListOrBuffer::PrpList(a) => *a,
            PrpListOrBuffer::Buffer(a) => *a,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            PrpListOrBuffer::PrpList(_) => true,
            PrpListOrBuffer::Buffer(_) => false,
        }
    }
}

impl<P: PhysicalPageCollection> VirtualRegion<P> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn new(
        virt: *mut u8,
        len: usize,
        _cache_type: CacheType,
        phys_page_list: P,
    ) -> Self {
        Self {
            len,
            virt,
            _cache_type,
            phys_page_list,
        }
    }

    pub fn base<T>(&self) -> *const T {
        self.virt as *const T
    }

    pub fn base_mut<T>(&mut self) -> *mut T {
        self.virt as *mut T
    }

    pub fn get_prp_list_or_buffer<D>(&mut self, dma: P::DmaType) -> Option<PrpListOrBuffer> {
        self.phys_page_list.get_prp_list_or_buffer(dma)
    }

    pub fn get_dptr(&mut self, sgl_allowed: bool) -> Option<Dptr> {
        self.phys_page_list.get_dptr(sgl_allowed)
    }
}
