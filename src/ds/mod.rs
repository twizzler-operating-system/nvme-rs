use std::time::Duration;

pub mod cmd;
pub mod controller;
pub mod identify;
pub mod namespace;
pub mod queue;
pub mod sgl;
pub mod status;
pub mod uuid;

pub type Address = u64;

pub type InterruptVector = u16;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OneHundredMilliseconds(u16);

impl From<OneHundredMilliseconds> for Duration {
    fn from(o: OneHundredMilliseconds) -> Self {
        Duration::from_millis(o.0 as u64 * 100)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Microseconds(u32);

impl From<Microseconds> for Duration {
    fn from(m: Microseconds) -> Self {
        Duration::from_micros(m.0 as u64)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Minutes(u16);

impl From<Minutes> for Duration {
    fn from(m: Minutes) -> Self {
        Duration::from_millis(m.0 as u64 * 1000 * 60)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seconds(u8);

impl From<Seconds> for Duration {
    fn from(m: Seconds) -> Self {
        Duration::from_millis(m.0 as u64 * 1000)
    }
}

use modular_bitfield::prelude::*;

#[bitfield]
#[derive(BitfieldSpecifier, Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HalfSeconds(u8);

impl From<HalfSeconds> for Duration {
    fn from(m: HalfSeconds) -> Self {
        Duration::from_millis(m.get_0() as u64 * 500)
    }
}
