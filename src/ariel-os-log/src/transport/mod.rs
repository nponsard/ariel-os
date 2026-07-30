#[cfg(feature = "logging-over-generic-usb")]
pub mod generic_usb;
#[cfg(feature = "logging-over-generic-usb")]
pub(crate) use generic_usb::{flush, write_bytes};
