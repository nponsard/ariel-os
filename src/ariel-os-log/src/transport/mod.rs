#![allow(unsafe_code)]
cfg_select! {
    feature = "linked-logging-function" => {
        pub(crate) fn write_bytes(bytes: &[u8]) {
            unsafe extern "Rust" {
                fn __ariel_os_log_write_bytes(bytes: &[u8]);
            }
            unsafe { __ariel_os_log_write_bytes(bytes) }
        }

        #[cfg(feature = "defmt")]
        pub(crate) fn flush() {
            unsafe extern "Rust" {
                fn __ariel_os_log_flush();
            }
            unsafe { __ariel_os_log_flush() }
        }
    }
    _ => {
        mod dummy;
        #[allow(unused, reason = "conditional compilation")]
        pub(crate) use dummy::{write_bytes, flush};
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
