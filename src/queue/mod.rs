use crate::{ds::queue::comentry::CommonCompletion, host_memory::VirtualRegion};

pub struct SubmissionQueue<const STRIDE: usize> {
    tail: u16,
    head: u16,
    len: u16,
    memory: VirtualRegion,
}

impl<const STRIDE: usize> SubmissionQueue<STRIDE> {
    pub fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % self.len
    }

    pub fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    pub fn submit(&mut self, data: &[u8; STRIDE]) -> Option<u16> {
        if self.is_full() {
            return None;
        }
        let tail = self.tail;
        self.tail = (self.tail + 1) % self.len;
        let ptr = self.get_entry_pointer(tail);
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr, STRIDE) };
        slice.copy_from_slice(data);
        Some(tail)
    }

    fn get_entry_pointer(&mut self, ent: u16) -> *mut u8 {
        unsafe { self.memory.base_mut::<u8>().add(ent as usize * STRIDE) }
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
        STRIDE
    }
}

pub struct CompletionQueue<const STRIDE: usize> {
    head: u16,
    len: u16,
    phase: bool,
    memory: VirtualRegion,
}

impl<const STRIDE: usize> CompletionQueue<STRIDE> {
    pub fn stride(&self) -> usize {
        STRIDE
    }

    fn get_entry_pointer(&self, ent: u16) -> *const u8 {
        unsafe { self.memory.base::<u8>().add(ent as usize * STRIDE) }
    }

    fn get_entry_compl_pointer(&self, ent: u16) -> *const CommonCompletion {
        let ptr = self.get_entry_pointer(ent);
        ptr as *const CommonCompletion
    }

    fn get_entry_slice(&self, ent: u16) -> &[u8] {
        let ptr = self.get_entry_pointer(ent);
        unsafe { core::slice::from_raw_parts(ptr, STRIDE) }
    }

    pub fn ready(&self) -> bool {
        let entry = unsafe {
            self.get_entry_compl_pointer(self.head)
                .as_ref()
                .unwrap_unchecked()
        };
        entry.phase() != self.phase
    }

    pub fn get_completion(&mut self, output: &mut [u8; STRIDE]) -> Option<u16> {
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
        Some(head)
    }
}
