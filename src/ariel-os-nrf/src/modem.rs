use ariel_os_debug::log::{info,debug};
use embassy_nrf::bind_interrupts;
use embassy_nrf::interrupt::Interrupt;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::interrupt::Priority;
use embassy_nrf::interrupt::typelevel;
use embassy_nrf::pac;
use embassy_nrf::pac::NVMC_S;
use embassy_nrf::pac::UICR_S;
use embassy_nrf::pac::nvmc::vals::Wen;
use embassy_nrf::pac::uicr::vals::{Hfxocnt, Hfxosrc};
use embassy_nrf::peripherals;
use tinyrlibc::*;
#[doc(hidden)]
pub struct InterruptHandler {
    _private: (),
}

impl typelevel::Handler<typelevel::IPC> for InterruptHandler {
    unsafe fn on_interrupt() {
        // info!("IPC interrupt triggered");
        nrf_modem::ipc_irq_handler();
    }
}

bind_interrupts!(struct Irqs{
    IPC => InterruptHandler;
});

unsafe extern "C" {
    static _MODEM_start: u8;
    static _MODEM_length: u8;
}

fn uicr_hfxo_workaround() {
    let uicr = embassy_nrf::pac::UICR_S;
    let hfxocnt = uicr.hfxocnt().read().hfxocnt().to_bits();
    let hfxosrc = uicr.hfxosrc().read().hfxosrc().to_bits();

    if hfxocnt != 255 && hfxosrc != 1 {
        return;
    }

    let irq_disabled = cortex_m::register::primask::read().is_inactive();
    if !irq_disabled {
        cortex_m::interrupt::disable();
    }
    cortex_m::asm::dsb();
    while !NVMC_S.ready().read().ready() {}

    NVMC_S
        .config()
        .write(|w| w.set_wen(pac::nvmc::vals::Wen::WEN));
    while !NVMC_S.ready().read().ready() {}

    if hfxosrc == 1 {
        UICR_S.hfxosrc().write(|w| w.set_hfxosrc(Hfxosrc::TCXO));
        cortex_m::asm::dsb();
        while !NVMC_S.ready().read().ready() {}
    }

    if hfxocnt == 255 {
        UICR_S.hfxocnt().write(|w| w.set_hfxocnt(Hfxocnt(32)));
        cortex_m::asm::dsb();
        while !NVMC_S.ready().read().ready() {}
    }

    NVMC_S
        .config()
        .write(|w| w.set_wen(pac::nvmc::vals::Wen::REN));
    while !NVMC_S.ready().read().ready() {}

    if !irq_disabled {
        unsafe {
            cortex_m::interrupt::enable();
        }
    }

    cortex_m::peripheral::SCB::sys_reset();
}

#[doc(hidden)]
pub fn driver() {
    use cortex_m::peripheral::NVIC;
    embassy_nrf::interrupt::IPC.set_priority(Priority::P0);

    unsafe {
        NVIC::unmask(Interrupt::IPC);
    }
    let clock = embassy_nrf::pac::CLOCK_S;
    // clock.tasks_hfclkstart().write_value(1);
    let stat = clock.hfclkstat().read();
    defmt::info!("HFCLK Source: {}", stat.src().to_bits());
    defmt::info!("HFCLK State: {}", stat.state());

    let uicr = embassy_nrf::pac::UICR_S;
    let hfxocnt = uicr.hfxocnt().read().hfxocnt().to_bits();
    let hfxosrc = uicr.hfxosrc().read().hfxosrc().to_bits();

    defmt::info!("HFXO Count: {}", hfxocnt);
    defmt::info!("HFXO Source: {}", hfxosrc);

    uicr_hfxo_workaround();

    let hfxocnt = uicr.hfxocnt().read().hfxocnt().to_bits();
    let hfxosrc = uicr.hfxosrc().read().hfxosrc().to_bits();

    defmt::info!("HFXO Count: {}", hfxocnt);
    defmt::info!("HFXO Source: {}", hfxosrc);
    fn configure_modem_non_secure() -> u32 {
        // The RAM memory space is divided into 32 regions of 8 KiB.
        // Set IPC RAM to nonsecure
        const SPU_REGION_SIZE: u32 = 0x2000; // 8kb
        const RAM_START: u32 = 0x2000_0000; // 256kb
        let ipc_start: u32 = unsafe { &_MODEM_start as *const u8 } as u32;
        let ipc_reg_offset = (ipc_start - RAM_START) / SPU_REGION_SIZE;
        let ipc_reg_count = (unsafe { &_MODEM_length as *const u8 } as u32) / SPU_REGION_SIZE;
        let spu = embassy_nrf::pac::SPU;
        let range = ipc_reg_offset..(ipc_reg_offset + ipc_reg_count);
        debug!("marking region as non secure: {}..{}", range.start, range.end);
        for i in range {
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
        ipc_start
    }
    let ipc_start = configure_modem_non_secure();
}
