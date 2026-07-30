//! Connector to custom logging transports.
//!
//! ## For implementors
//!
//!  Custom logging transport implementations end up depending on the HAL, that then depends on
//! `ariel-os-log`, creating a circular dependency.
//! To break this circle, the custom implementations have to register their `write_bytes` and `flush`
//! implementation with [`register_transport_fns`]

cfg_select! {
    feature = "custom-transport" => {
        mod custom;
        #[cfg(feature = "defmt")]
        pub (crate) use custom::flush;
        pub (crate) use custom::write_bytes;
        pub use custom::register_transport_fns;
    }
    _ => {
        mod dummy;
        #[cfg(feature = "defmt")]
        pub (crate) use dummy::flush;
        pub (crate) use dummy::write_bytes;
        pub use dummy::register_transport_fns;
    }
}

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
