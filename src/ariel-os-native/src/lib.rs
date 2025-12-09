//! Items specific to the "native" implementation.

#![cfg_attr(nightly, feature(doc_cfg))]

#[cfg(feature = "hwrng")]
pub mod hwrng;

pub mod identity;
pub mod peripherals {}

pub struct OptionalPeripherals {}

#[must_use]
pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {}
}

pub struct SWI {}
