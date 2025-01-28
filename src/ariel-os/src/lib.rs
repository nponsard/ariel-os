//! Ariel OS is an operating system for secure, memory-safe, low-power Internet of Things (IoT).
//! Supported hardware includes various 32-bit microcontrollers.
//!
//! This is the API documentation for Ariel OS.
//! Other resources available are:
//! - 📔 Extensive documentation for Ariel OS can be found in the
//!   [book](https://ariel-os.github.io/ariel-os/dev/docs/book/).
//! - ⚙️  The git repository is available on
//!   [GitHub](https://github.com/ariel-os/ariel-os).
//! - ✨ [Examples](https://github.com/ariel-os/ariel-os/tree/main/examples)
//!   demonstrates various features of Ariel OS.
//! - 🧪 A set of [test cases](https://github.com/ariel-os/ariel-os/tree/main/tests)
//!   further verifies the capabilities of Ariel OS.
//! - 🚧 The [roadmap](https://github.com/ariel-os/ariel-os/issues/242)
//!   shows the planned features for Ariel OS.
//!
//! # Cargo features
//!
//!  Ariel OS is highly modular with a significant number of features
//!  to configure the operating system.
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
#![no_std]
#![feature(doc_auto_cfg)]

#[cfg(feature = "bench")]
#[doc(inline)]
pub use ariel_os_bench as bench;
#[doc(inline)]
pub use ariel_os_buildinfo as buildinfo;
#[cfg(feature = "coap")]
#[doc(inline)]
pub use ariel_os_coap as coap;
#[doc(inline)]
pub use ariel_os_debug as debug;
#[doc(inline)]
pub use ariel_os_identity as identity;
#[cfg(feature = "random")]
#[doc(inline)]
pub use ariel_os_random as random;
#[cfg(feature = "storage")]
#[doc(inline)]
pub use ariel_os_storage as storage;
#[cfg(feature = "threading")]
#[doc(inline)]
pub use ariel_os_threads as thread;

// Attribute macros
pub use ariel_os_macros::config;
pub use ariel_os_macros::spawner;
pub use ariel_os_macros::task;
#[cfg(any(feature = "threading", doc))]
pub use ariel_os_macros::thread;

// ensure this gets linked
use ariel_os_boards as _;

pub use ariel_os_embassy::api::*;

/// This module contains all third party crates as used by Ariel OS.
///
/// TODO: The version of this crate (`ariel-os`) will correspond to changes in
/// these dependencies (keeping semver guarantees).
pub mod reexports {
    pub use ariel_os_embassy::reexports::*;
    // These are used by proc-macros we provide
    pub use linkme;
    pub use static_cell;
}
