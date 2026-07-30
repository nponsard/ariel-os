#[cfg(feature = "logging-over-generic-usb")]
pub mod generic_usb;
#[cfg(feature = "logging-over-generic-usb")]
pub(crate) use generic_usb::{flush, write_bytes};
#[cfg(feature = "logging-over-esp-jtag-serial")]
pub mod esp_jtag_serial;
#[cfg(feature = "logging-over-esp-jtag-serial")]
pub(crate) use esp_jtag_serial::{flush, write_bytes};

struct Transport;

impl core::fmt::Write for Transport {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        write_bytes(bytes);
        Ok(())
    }
}

// Based on <https://blog.m-ou.se/format-args/>.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write as _;

    Transport.write_fmt(args).unwrap();
}

#[doc(hidden)]
#[macro_export]
macro_rules! transport_println {
    ($($arg:tt)*) => {{
        #[expect(clippy::used_underscore_items, reason = "consistency with std::println")]
        $crate::transport::_print(format_args!("{}\n", format_args!($($arg)*)));
    }};
}
