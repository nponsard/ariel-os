use embassy_stm32::rng::Rng;
use embassy_stm32::{bind_interrupts, peripherals, rng};

#[cfg(not(any(capability = "hw/stm32-hash-rng", capability = "hw/stm32-rng")))]
compile_error!("no stm32 RNG capability enabled");

bind_interrupts!(struct Irqs {
    #[cfg(capability = "hw/stm32-aes-rng")]
    AES_RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-aes-rng-lpuart1")]
    AES_RNG_LPUART1 => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-hash-rng")]
    HASH_RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng")]
    RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng-cryp")]
    RNG_CRYP => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng-lpuart1")]
    RNG_LPUART1 => rng::InterruptHandler<peripherals::RNG>;
});

pub fn construct_rng(peripherals: &mut crate::OptionalPeripherals) {
    cfg_if::cfg_if! {
        // The union of all contexts that wind up in a construct_rng should be synchronized
        // with laze-project.yml's hwrng module.
        if #[cfg(any(context = "stm32"))] {
            let rng = Rng::new(
                peripherals
                    .RNG
                    // We don't even have to take it out, just use it to seed the RNG
                    .as_mut()
                    .expect("RNG has not been previously used"),
                Irqs,
            );

            ariel_os_random::construct_rng(rng);
        } else if #[cfg(context = "ariel-os")] {
            compile_error!("hardware RNG is not supported on this MCU family");
        }
    }
}
