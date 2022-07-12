#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct NamespaceId(u32);

pub struct NamespaceList<'a, const bytes: usize> {
    data: &'a [u8; bytes],
}

impl<'a, const bytes: usize> NamespaceList<'a, bytes> {
    pub fn new(data: &'a [u8; bytes]) -> Self {
        Self { data }
    }

    pub fn nr_bytes(&self) -> usize {
        bytes
    }
}

pub struct NamespaceListIter<'a, const bytes: usize> {
    list: NamespaceList<'a, bytes>,
    pos: usize,
}

impl<'a, const bytes: usize> Iterator for NamespaceListIter<'a, bytes> {
    type Item = NamespaceId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= bytes {
            return None;
        }
        let by = self.list.data[self.pos..(self.pos + 4)].as_ptr() as *const u32;
        let val = unsafe { *by };
        self.pos += 4;
        Some(NamespaceId(val))
    }
}

impl<'a, const bytes: usize> IntoIterator for NamespaceList<'a, bytes> {
    type Item = NamespaceId;

    type IntoIter = NamespaceListIter<'a, bytes>;

    fn into_iter(self) -> Self::IntoIter {
        NamespaceListIter { list: self, pos: 0 }
    }
}
