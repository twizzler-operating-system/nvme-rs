use super::Address;

pub mod admin;

pub enum PrpListOrBuffer {
    PrpList(Address),
    PrpFirstAndList(Address, Address),
    Buffer(Address),
}
