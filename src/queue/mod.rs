use std::mem::{size_of, MaybeUninit};

use crate::ds::queue::comentry::CommonCompletion;

pub struct SubmissionQueue {
    tail: u16,
    head: u16,
    len: u16,
    stride: usize,
    memory: *mut u8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CreateQueueError {
    MemoryRegionTooSmall,
    StrideTooSmall,
}

impl SubmissionQueue {
    pub fn new(
        memory: &mut [u8],
        nr_entries: u16,
        stride: usize,
    ) -> Result<Self, CreateQueueError> {
        if nr_entries as usize * stride > memory.len() {
            return Err(CreateQueueError::MemoryRegionTooSmall);
        }
        if stride < 1 {
            return Err(CreateQueueError::StrideTooSmall);
        }
        Ok(Self {
            head: 0,
            tail: 0,
            len: nr_entries,
            stride,
            memory: memory.as_mut_ptr(),
        })
    }

    pub fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.len
    }

    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    pub fn submit_bytes(&mut self, data: &[u8]) -> Option<u16> {
        if data.len() > self.stride {
            panic!("tried to submit data too big for stride");
        }
        if self.is_full() {
            return None;
        }
        let tail = self.tail;
        self.tail = (self.tail + 1) % self.len;
        let ptr = self.get_entry_pointer(tail);
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr, data.len()) };
        slice.copy_from_slice(data);
        Some(self.tail)
    }

    pub fn submit<T: Copy>(&mut self, data: &T) -> Option<u16> {
        let bytes = data as *const T as *const u8;
        let len = size_of::<T>();
        self.submit_bytes(unsafe { core::slice::from_raw_parts(bytes, len) })
    }

    fn get_entry_pointer(&mut self, ent: u16) -> *mut u8 {
        unsafe { self.memory.add(ent as usize * self.stride) }
    }

    pub fn update_head(&mut self, head: u16) {
        if head >= self.len {
            panic!("tried to set head to {} (len = {})", head, self.len);
        }
        self.head = head;
    }

    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn stride(&self) -> usize {
        self.stride
    }
}

pub struct CompletionQueue {
    head: u16,
    len: u16,
    phase: bool,
    stride: usize,
    memory: *const u8,
}

impl CompletionQueue {
    pub fn new(memory: &[u8], nr_entries: u16, stride: usize) -> Result<Self, CreateQueueError> {
        if nr_entries as usize * stride > memory.len() {
            return Err(CreateQueueError::MemoryRegionTooSmall);
        }
        if stride < core::mem::size_of::<CommonCompletion>() {
            return Err(CreateQueueError::StrideTooSmall);
        }
        Ok(Self {
            head: 0,
            len: nr_entries,
            phase: false,
            stride,
            memory: memory.as_ptr(),
        })
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    fn get_entry_pointer(&self, ent: u16) -> *const u8 {
        unsafe { self.memory.add(ent as usize * self.stride) }
    }

    fn get_entry_compl_pointer(&self, ent: u16) -> *const CommonCompletion {
        let ptr = self.get_entry_pointer(ent);
        ptr as *const CommonCompletion
    }

    fn get_entry_slice(&self, ent: u16) -> &[u8] {
        let ptr = self.get_entry_pointer(ent);
        unsafe { core::slice::from_raw_parts(ptr, self.stride) }
    }

    pub fn ready(&self) -> bool {
        let entry = unsafe {
            self.get_entry_compl_pointer(self.head)
                .as_ref()
                .unwrap_unchecked()
        };
        entry.phase() != self.phase
    }

    pub fn get_completion_bytes(&mut self, output: &mut [u8]) -> Option<u16> {
        if output.len() != self.stride {
            panic!("completion output bytes too small");
        }
        // TODO: volatile?
        let head = self.head;
        let entry = unsafe {
            self.get_entry_compl_pointer(head)
                .as_ref()
                .unwrap_unchecked()
        };
        if entry.phase() == self.phase {
            return None;
        }

        output.copy_from_slice(self.get_entry_slice(head));

        self.head = (head + 1) % self.len;
        if self.head == 0 {
            self.phase = !self.phase;
        }
        Some(self.head)
    }

    pub fn get_completion<T: Copy>(&mut self) -> Option<(u16, T)> {
        let mut data = MaybeUninit::uninit();
        let bytes = data.as_mut_ptr() as *mut u8;
        let len = size_of::<T>();
        let head =
            self.get_completion_bytes(unsafe { core::slice::from_raw_parts_mut(bytes, len) })?;

        Some((head, unsafe { data.assume_init() }))
    }
}
