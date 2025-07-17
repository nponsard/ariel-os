//! Items specific to the "native" implementation

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

pub struct OptionalPeripherals {}

pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {}
}

pub struct SWI {}
