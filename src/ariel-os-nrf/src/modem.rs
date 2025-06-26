use embassy_nrf::interrupt::typelevel;
use embassy_nrf::peripherals;
use embassy_nrf::bind_interrupts;
use tinyrlibc::*;

#[doc(hidden)]
pub struct InterruptHandler {
    _private: ()
}

impl typelevel::Handler<typelevel::IPC> for InterruptHandler {
    unsafe fn on_interrupt() {
        nrf_modem::ipc_irq_handler();
    }
}

bind_interrupts!(struct Irqs{
    IPC => InterruptHandler;
});
