#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct NamespaceId(u32);

impl NamespaceId {
    pub fn new(id: impl Into<u32>) -> Self {
        Self(id.into())
    }
}

pub struct NamespaceList<'a, const BYTES: usize> {
    data: &'a [u8; BYTES],
}

impl<'a, const BYTES: usize> NamespaceList<'a, BYTES> {
    pub fn new(data: &'a [u8; BYTES]) -> Self {
        Self { data }
    }

    pub fn nr_bytes(&self) -> usize {
        BYTES
    }
}

pub struct NamespaceListIter<'a, const BYTES: usize> {
    list: NamespaceList<'a, BYTES>,
    pos: usize,
}

impl<'a, const BYTES: usize> Iterator for NamespaceListIter<'a, BYTES> {
    type Item = NamespaceId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= BYTES {
            return None;
        }
        let by = self.list.data[self.pos..(self.pos + 4)].as_ptr() as *const u32;
        let val = unsafe { *by };
        if val == 0 {
            return None;
        }
        self.pos += 4;
        Some(NamespaceId(val))
    }
}

impl<'a, const BYTES: usize> IntoIterator for NamespaceList<'a, BYTES> {
    type Item = NamespaceId;

    type IntoIter = NamespaceListIter<'a, BYTES>;

    fn into_iter(self) -> Self::IntoIter {
        NamespaceListIter { list: self, pos: 0 }
    }
}
