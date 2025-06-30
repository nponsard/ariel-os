use ariel_os_debug::log::info;
use embassy_nrf::bind_interrupts;
use embassy_nrf::interrupt::typelevel;
use embassy_nrf::peripherals;
use tinyrlibc::*;

#[doc(hidden)]
pub struct InterruptHandler {
    _private: (),
}

impl typelevel::Handler<typelevel::IPC> for InterruptHandler {
    unsafe fn on_interrupt() {
        nrf_modem::ipc_irq_handler();
    }
}

bind_interrupts!(struct Irqs{
    IPC => InterruptHandler;
});

#[doc(hidden)]
pub fn driver() {
    const SPU_REGION_SIZE: u32 = 0x2000; // 8kb
    const RAM_START: u32 = 0x2000_0000; // 256kb
    let spu = embassy_nrf::pac::SPU;
    let region_start = 0x2000_0000 - RAM_START / SPU_REGION_SIZE;
    let region_end = region_start + (0x2000_8000 - 0x2000_0000) / SPU_REGION_SIZE;
    info!("region_start: {:#x}, region_end: {:#x}", region_start, region_end);

    for i in 0..10 {
        spu.ramregion(i as usize).perm().write(|w| {
            w.set_execute(true);
            w.set_write(true);
            w.set_read(true);
            w.set_secattr(false);
            w.set_lock(false);
        })
    }

    // Set regulator access registers to nonsecure
    spu.periphid(4).perm().write(|w| w.set_secattr(false));
    // Set clock and power access registers to nonsecure
    spu.periphid(5).perm().write(|w| w.set_secattr(false));
    // Set IPC access register to nonsecure
    spu.periphid(42).perm().write(|w| w.set_secattr(false));
}
