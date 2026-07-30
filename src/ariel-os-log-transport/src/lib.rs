//! Provides logging transport adapters.

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

cfg_select! {
    feature = "writer" => {
        mod writer;
        pub use writer::init;
    }
    _ => {
        mod dummy;
        pub use dummy::init;
    }
}
