use ariel_os_debug::log::{info,error};
use embassy_nrf::bind_interrupts;
use embassy_nrf::interrupt::Interrupt;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::interrupt::Priority;
use embassy_nrf::interrupt::typelevel;
use embassy_nrf::pac;
use embassy_nrf::peripherals;
use nrf_modem::ConnectionPreference;
use nrf_modem::SystemMode;
use tinyrlibc::*;

use cortex_m::peripheral::NVIC;

#[doc(hidden)]
pub async fn driver() {
    info!("Initializing nRF Modem driver");
    embassy_nrf::interrupt::IPC.set_priority(Priority::P1);
//   let a = nrf_modem::init(SystemMode {
//         lte_support: true,
//         lte_psm_support: true,
//         nbiot_support: true,
//         gnss_support: true,
//         preference: ConnectionPreference::None,
//     })
//     .await;

//     if a.is_err() {
//         error!("Failed to initialize modem: {:?}", a);
//         // exit(ExitCode::FAILURE);
//     }
}

#[doc(hidden)]
pub struct InterruptHandler {
    _private: (),
}

impl typelevel::Handler<typelevel::IPC> for InterruptHandler {
    unsafe fn on_interrupt() {
        panic!("IPC interrupt handler called, this should not happen in the nRF Modem driver");
        info!("IPC interrupt triggered");
        nrf_modem::ipc_irq_handler();
    }
}

bind_interrupts!(struct Irqs{
    IPC => InterruptHandler;
});
