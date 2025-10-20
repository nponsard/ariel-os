#![expect(unsafe_code)]

use embassy_nrf::{
    interrupt, pac,
    pac::{
        NVMC_S, UICR_S,
        uicr::vals::{Hfxocnt, Hfxosrc},
    },
};
use nrf_modem::{ConnectionPreference, MemoryLayout, SystemMode};
// need to make the symbols available so nrf_modem can link against them
extern crate tinyrlibc as _;

use ariel_os_debug::log::debug;

#[cfg(feature = "executor-interrupt")]
use cortex_m::interrupt::InterruptNumber as _;

#[interrupt]
fn IPC() {
    nrf_modem::ipc_irq_handler();
}

unsafe extern "C" {
    static _MODEM_start: u8;
    static _MODEM_length: u8;
}

// Workaround used in the nrf mdk: file system_nrf91.c , function SystemInit(), after `#if !defined(NRF_SKIP_UICR_HFXO_WORKAROUND)`
// Use this until commit `3e2b23d2f453d10324896484f9d045d2821bd567` is included in the embassy-nrf version we use.
fn uicr_hfxo_workaround() {
    let uicr = embassy_nrf::pac::UICR_S;
    let hfxocnt = uicr.hfxocnt().read().hfxocnt().to_bits();
    let hfxosrc = uicr.hfxosrc().read().hfxosrc().to_bits();

    if hfxocnt != 255 && hfxosrc != 1 {
        return;
    }

    cortex_m::interrupt::free(|_| {
        cortex_m::asm::dsb();
        while !NVMC_S.ready().read().ready() {}

        NVMC_S
            .config()
            .write(|w| w.set_wen(pac::nvmc::vals::Wen::WEN));
        while !NVMC_S.ready().read().ready() {}

        UICR_S.hfxosrc().write(|w| w.set_hfxosrc(Hfxosrc::TCXO));
        cortex_m::asm::dsb();
        while !NVMC_S.ready().read().ready() {}

        UICR_S.hfxocnt().write(|w| w.set_hfxocnt(Hfxocnt(32)));
        cortex_m::asm::dsb();
        while !NVMC_S.ready().read().ready() {}

        NVMC_S
            .config()
            .write(|w| w.set_wen(pac::nvmc::vals::Wen::REN));
        while !NVMC_S.ready().read().ready() {}
    });

    cortex_m::peripheral::SCB::sys_reset();
}

#[doc(hidden)]
pub async fn driver() {
    uicr_hfxo_workaround();

    // from https://github.com/diondokter/nrf-modem/issues/31
    // allows us to use the modem in S mode
    fn configure_modem_non_secure() -> u32 {
        // The RAM memory space is divided into 32 regions of 8 KiB.
        // Set IPC RAM to nonsecure
        const SPU_REGION_SIZE: u32 = 0x2000; // 8 KiB
        const RAM_START: u32 = 0x2000_0000; // 256 KiB of RAM. This is the start address of the physical RAM. Symbol is defined in ariel-os-rt.
        // SAFETY: the linker must link this symbol with a valid address in RAM region.
        let ipc_start: u32 = unsafe { &_MODEM_start as *const u8 } as u32;
        let ipc_reg_offset = (ipc_start - RAM_START) / SPU_REGION_SIZE;
        // SAFETY: the linker must link this symbol with a valid length that does not exceed the nrf91 available RAM. Symbol is defined in ariel-os-rt.
        let ipc_reg_count = (unsafe { &_MODEM_length as *const u8 } as u32) / SPU_REGION_SIZE;
        let spu = embassy_nrf::pac::SPU;
        let range = ipc_reg_offset..(ipc_reg_offset + ipc_reg_count);
        debug!(
            "marking region as non secure: {}..{}",
            range.start, range.end
        );
        for i in range {
            spu.ramregion(i as usize).perm().write(|w| {
                w.set_execute(false);
                w.set_write(true);
                w.set_read(true);
                w.set_secattr(false);
                w.set_lock(false);
            })
        }

        // Set needed peripherals to nonsecure

        // Section 4.2.2 of the nRF9160 and nRF9151 Product Specification
        const REGULATORS_PERIPHERAL_ID: usize = 4;
        const CLOCK_POWER_PERIPHERAL_ID: usize = 5;
        const IPC_PERIPHERAL_ID: usize = 42;

        spu.periphid(REGULATORS_PERIPHERAL_ID)
            .perm()
            .write(|w| w.set_secattr(false));
        spu.periphid(CLOCK_POWER_PERIPHERAL_ID)
            .perm()
            .write(|w| w.set_secattr(false));
        spu.periphid(IPC_PERIPHERAL_ID)
            .perm()
            .write(|w| w.set_secattr(false));
        ipc_start
    }
    let ipc_start = configure_modem_non_secure();

    let system_mode = SystemMode {
        lte_support: true,
        lte_psm_support: true,
        nbiot_support: false,
        gnss_support: true,
        preference: ConnectionPreference::None,
    };

    // Set the base address for the IPC shared memory, use default sizes.
    let memory_layout = MemoryLayout {
        base_address: ipc_start,
        ..Default::default()
    };

    #[cfg(feature = "executor-interrupt")]
    nrf_modem::init_with_custom_layout(system_mode, memory_layout, crate::SWI.number() as u8)
        .await
        .unwrap();
    #[cfg(not(feature = "executor-interrupt"))]
    nrf_modem::init_with_custom_layout(system_mode, memory_layout)
        .await
        .unwrap();
}
