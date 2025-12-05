#![expect(unsafe_code)]

use ariel_os_debug::log::debug;
use ariel_os_threads::{THREAD_FNS, start_threading};

/// # Safety
///
/// The caller must ensure that this function is only called once.
pub unsafe fn start() -> ! {
    let mut count = 0;

    for thread_fn in THREAD_FNS {
        count += 1;
        thread_fn();
    }

    debug!("{} threads started", count);

    // SAFETY: this function must only be called once, enforced by caller
    unsafe {
        start_threading();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
