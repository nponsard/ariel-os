//! Provides power management functionality.
#![deny(clippy::pedantic)]
#![deny(missing_docs)]
#![no_std]

#[cfg(context = "cortex-m")]
use cortex_m::peripheral::SCB;

#[cfg(any(
    context = "esp32",
    context = "esp32c3",
    context = "esp32c6",
    context = "esp32s3"
))]
use esp_hal::reset::software_reset;

/// Reboot the MCU
/// This function initiates a software reset of the microcontroller.
///
/// # Note
///
/// This function never returns as it triggers an immediate system reset.
///
/// # Panics
///
/// … if called on an unsupported platform.
pub fn reboot() -> ! {
    #[cfg(context = "cortex-m")]
    {
        SCB::sys_reset()
    }
    #[cfg(any(
        context = "esp32",
        context = "esp32c3",
        context = "esp32c6",
        context = "esp32s3"
    ))]
    {
        software_reset()
    }

    #[cfg(all(
        not(any(
            context = "cortex-m",
            context = "esp32",
            context = "esp32c3",
            context = "esp32c6",
            context = "esp32s3"
        )),
        not(feature = "doc"),
        not(feature = "no-boards")
    ))]
    {
        compile_error!("reboot is not yet implemented for this platform")
    }

    #[allow(unreachable_code, reason = "fallback loop")]
    loop {
        core::hint::spin_loop();
    }
}
