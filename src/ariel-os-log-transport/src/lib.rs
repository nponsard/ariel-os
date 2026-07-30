//! Provides logging transport adapters.

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

cfg_select! {
    feature = "writer" => {
        mod writer;
        pub use writer::init;
        #[allow(unused)]
        use writer::{write_bytes,flush};
    }
    _ => {
        mod dummy;
        pub use dummy::init;
        #[allow(unused)]
        use dummy::{write_bytes,flush};
    }
}

#[cfg(feature = "external")]
mod external {
    #![allow(unsafe_code)]
    use crate::{flush, write_bytes};
    #[unsafe(no_mangle)]
    pub extern "Rust" fn __ariel_os_log_write_bytes(bytes: &[u8]) {
        write_bytes(bytes);
    }

    #[unsafe(no_mangle)]
    pub extern "Rust" fn __ariel_os_log_flush() {
        flush();
    }
}
