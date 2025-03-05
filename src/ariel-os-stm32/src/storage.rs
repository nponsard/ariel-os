// TODO: we may later want to introduce a Cargo feature for async flash drivers, possibly enabled
// by laze.

use embassy_stm32::flash;

#[cfg(capability = "async-flash-driver")]
embassy_stm32::bind_interrupts!(struct Irqs {
    FLASH => flash::InterruptHandler;
});

#[cfg(capability = "async-flash-driver")]
pub type Flash = flash::Flash<'static>;
#[cfg(not(capability = "async-flash-driver"))]
pub type Flash =
    embassy_embedded_hal::adapter::BlockingAsync<flash::Flash<'static, flash::Blocking>>;
pub type FlashError = flash::Error;

pub fn init(peripherals: &mut crate::OptionalPeripherals) -> Flash {
    #[cfg(capability = "async-flash-driver")]
    let flash = flash::Flash::new(peripherals.FLASH.take().unwrap(), Irqs);
    #[cfg(not(capability = "async-flash-driver"))]
    let flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash::Flash::new_blocking(
        peripherals.FLASH.take().unwrap(),
    ));
    flash
}
