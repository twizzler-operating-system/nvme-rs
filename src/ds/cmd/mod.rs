use super::Address;

pub mod admin;

pub enum PrpListOrBuffer {
    PrpList(Address),
    PrpBuffer(Address),
}
