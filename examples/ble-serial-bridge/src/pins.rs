use ariel_os::hal::{peripherals, uart};

#[cfg(context = "nrf52832")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52832")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_12,
    uart_tx: P0_11,
});

#[cfg(context = "nrf52833")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52833")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_02,
    uart_tx: P0_03,
});

#[cfg(context = "nrf52840")]
pub type TestUart<'a> = uart::UARTE1<'a>;
#[cfg(context = "nrf52840")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_05,
    uart_tx: P0_03,
});

#[cfg(context = "rp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PIN_1,
    uart_tx: PIN_0,
});
