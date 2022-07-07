use modular_bitfield::prelude::*;

pub struct NamespaceIdentBuffer {
    data: [u8; 4096],
}

#[bitfield(bits = 32)]
struct NamespaceIdentDescriptorHead {
    #[skip(setters)]
    ident_type: u8,
    #[skip(setters)]
    ident_len: u8,
    #[skip]
    res: u16,
}

pub enum NamespaceDescriptor<'a> {
    IeeeExtended(&'a [u8]),
    NamespaceGUID(&'a [u8]),
    NamespaceUUID(&'a [u8]),
    CommandSetId(&'a [u8]),
}

pub struct NamespaceIdentBufferIterator<'a> {
    buf: &'a NamespaceIdentBuffer,
    pos: usize,
}

impl NamespaceIdentBuffer {
    pub fn iter(&self) -> NamespaceIdentBufferIterator<'_> {
        NamespaceIdentBufferIterator { buf: self, pos: 0 }
    }
}

impl<'a> Iterator for NamespaceIdentBufferIterator<'a> {
    type Item = NamespaceDescriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 4096 {
            return None;
        }
        let head =
            NamespaceIdentDescriptorHead::from_bytes(self.buf.data[0..4].try_into().unwrap());
        let desc_len = head.ident_len() as usize + 4;
        let loc = self.pos;
        self.pos += desc_len;
        if self.pos > 4096 {
            return None;
        }

        let slice = &self.buf.data[(loc + 4)..(loc + 4 + head.ident_len() as usize)];

        Some(match head.ident_type() {
            1 => NamespaceDescriptor::IeeeExtended(slice),
            2 => NamespaceDescriptor::NamespaceGUID(slice),
            3 => NamespaceDescriptor::NamespaceUUID(slice),
            4 => NamespaceDescriptor::CommandSetId(slice),
            _ => return self.next(),
        })
    }
}
