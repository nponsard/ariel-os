#![expect(unsafe_code)]

use crate::{Arch, SCHEDULER, Thread, cleanup};
use ariel_os_debug::log::{debug, info};
use core::arch::global_asm;
#[cfg(context = "esp32c6")]
use esp_hal::peripherals::INTPRI as SYSTEM;
#[cfg(context = "esp32c3")]
use esp_hal::peripherals::SYSTEM;
use esp_hal::{
    interrupt::{self},
    peripherals::Interrupt,
    riscv,
    system::Cpu as EspHalCpu,
};
use portable_atomic::Ordering;
pub struct Cpu;

static _CURRENT_CTX_PTR: portable_atomic::AtomicU32 = portable_atomic::AtomicU32::new(0);

static _NEXT_CTX_PTR: portable_atomic::AtomicU32 = portable_atomic::AtomicU32::new(0);

unsafe extern "C" {
    fn return_from_interrupt();
}

#[derive(Debug)]
pub struct ThreadData {
    saved_registers: [usize; 17],
}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = default_trap_frame();

    /// Triggers software interrupt for the context switch.
    fn schedule() {
        info!("risscv::schedule()");

        let mstatus_st = esp_hal::riscv::register::mstatus::read();

        let mstatus = mstatus_st.bits();

        info!(
            "mstatus.mie: {} mstatus.mpie: {} ",
            mstatus_st.mie(),
            mstatus_st.mpie()
        );
        unsafe {
            (&*SYSTEM::PTR)
                .cpu_intr_from_cpu(0)
                .modify(|_, w| w.cpu_intr().set_bit());
        }

        info!("mstatus: {:#x}", mstatus);
        // let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::min());
        // debug!("e : {:?}", e);
    }

    fn setup_stack(thread: &mut Thread, stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let stack_start = stack.as_ptr() as usize;
        // 16 byte alignment.
        let stack_pos = (stack_start + stack.len()) & 0xFFFFFFE0;
        // Set up PC, SP, RA and first argument for function.
        // sp
        thread.data.saved_registers[12] = stack_pos;
        // a0
        thread.data.saved_registers[13] = arg.unwrap_or_default();

        info!("cleanup addr: {}", cleanup as usize);
        // ra
        thread.data.saved_registers[14] = cleanup as usize;
        // pc
        thread.data.saved_registers[15] = func as usize;

        thread.stack_lowest = stack_start;
        thread.stack_highest = stack_pos;

        // Safety: This is the place to initialize stack painting.
        // unsafe { thread.stack_paint_init(stack_pos) };
    }

    /// Enable and trigger the appropriate software interrupt.
    fn start_threading() {
        debug!("riscv::start_threading");
        interrupt::disable(EspHalCpu::ProCpu, Interrupt::FROM_CPU_INTR0);
        debug!("interrupts disabled");

        Self::schedule();
        debug!("schedule done");
        // Panics if `FROM_CPU_INTR0` is among `esp_hal::interrupt::RESERVED_INTERRUPTS`,
        // which isn't the case.

        interrupt::enable_direct(
            Interrupt::FROM_CPU_INTR0,
            esp_hal::interrupt::Priority::Priority3,
            esp_hal::interrupt::CpuInterrupt::Interrupt20,
            FROM_CPU_INTR0,
        )
        .unwrap();
        let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::min());
        debug!("e : {:?}", e);
        debug!("interrupt enabled");
    }

    fn wfi() {
        riscv::asm::wfi();
    }
}

const fn default_trap_frame() -> ThreadData {
    ThreadData {
        saved_registers: [0usize; 17],
    }
}

// global_asm!(
//     r#"

//     .section .trap.rust, "ax"          // FIXME: is this right ?
//     .globl FROM_CPU_INTR0
//     .align 4
//     FROM_CPU_INTR0:

//         // unset mie
//         // csrc mstatus, 0x8

//         call {sched}

//         // if a0 is null, no need to save
//         beqz    a0, restore
//         // save registers
//         sw s0, 0(a0)
//         sw s1, 4(a0)
//         sw s2, 8(a0)
//         sw s3, 12(a0)
//         sw s4, 16(a0)
//         sw s5, 20(a0)
//         sw s6, 24(a0)
//         sw s7, 28(a0)
//         sw s8, 32(a0)
//         sw s9, 36(a0)
//         sw s10, 40(a0)
//         sw s11, 44(a0)
//         sw sp, 48(a0)
//         sw a0, 52(a0)
//         sw ra, 56(a0)

//         csrr t0, mepc
//         csrr t1, mstatus

//         sw t0, 60(a0)
//         sw t1, 64(a0)

//     restore:
//         // load registers
//         lw s0, 0(a1)
//         lw s1, 4(a1)
//         lw s2, 8(a1)
//         lw s3, 12(a1)
//         lw s4, 16(a1)
//         lw s5, 20(a1)
//         lw s6, 24(a1)
//         lw s7, 28(a1)
//         lw s8, 32(a1)
//         lw s9, 36(a1)
//         lw s10, 40(a1)
//         lw s11, 44(a1)

//         lw sp, 48(a1)
//         lw a0, 52(a1)
//         // lw ra, 56(a1)
//         lw t0, 60(a1)
//         csrw mepc,t0
//         lw t1, 64(a1)
//         csrw mstatus, t1

//         // set mpie an mie
//         csrr t0, mstatus
//         ori t0, t0, 0x88
//         csrw mstatus, t0

//         mret
//     "#,
//     sched = sym sched
// );

/// Handler for software interrupt 0, which we use for context switching.
// SAFETY: symbol required by `esp-pacs`.
#[esp_hal::ram]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn FROM_CPU_INTR0() {
    debug!("interrupt !");
    unsafe {
        // clear FROM_CPU_INTR0
        (&*SYSTEM::PTR)
            .cpu_intr_from_cpu(0)
            .modify(|_, w| w.cpu_intr().clear_bit());

        let mstatus_st = esp_hal::riscv::register::mstatus::read();
        let mstatus = mstatus_st.bits();

        info!(
            "mstatus.mie: {} mstatus.mpie: {} ",
            mstatus_st.mie(),
            mstatus_st.mpie()
        );

        info!("mstatus: {:#x}", mstatus);
        let (current_high_regs, next_high_regs) = sched();

        _CURRENT_CTX_PTR.store(current_high_regs, Ordering::SeqCst);
        _NEXT_CTX_PTR.store(next_high_regs, Ordering::SeqCst);

        // set MPIE in MSTATUS to 0 to disable interrupts while task switching
        // esp_hal::riscv::register::mstatus::write(
        //     esp_hal::riscv::register::mstatus::Mstatus::from_bits(mstatus & !(1 << 7)),
        // );

        // load address of return_from_interrupt into MEPC - will run after all registers are restored
        esp_hal::riscv::register::mepc::write(return_from_interrupt as *const () as usize);

        debug!("return to {}", return_from_interrupt as *const () as usize);
        core::arch::asm!("mret", options(noreturn));
    }
}

/// Probes the runqueue for the next thread and switches context if needed.
///
/// # Safety
///
/// This method might switch the current register state that is saved in the
/// `trap_frame`.
/// It should only be called from inside the trap handler that is responsible for
/// context switching.
// #[unsafe(link_section = ".trap.rust")]
#[unsafe(no_mangle)]
unsafe extern "C" fn sched() -> (u32, u32) {
    info!("sched !");

    let mstatus = esp_hal::riscv::register::mstatus::read().bits();

    let (current_high_regs, next_high_regs) = loop {
        if let Some(res) = SCHEDULER.with_mut(|mut scheduler| {
            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    Cpu::wfi();
                    return None;
                }
            };

            let mut current_high_regs = core::ptr::null();

            if let Some(current_tid_ref) = scheduler.current_tid_mut() {
                if next_tid == *current_tid_ref {
                    return Some((0, 0));
                }
                let current_tid = *current_tid_ref;
                *current_tid_ref = next_tid;
                let current = scheduler.get_unchecked_mut(current_tid);
                current_high_regs = current.data.saved_registers.as_ptr();
            } else {
                *scheduler.current_tid_mut() = Some(next_tid);
            }
            let next = scheduler.get_unchecked_mut(next_tid);
            next.data.saved_registers[16] = mstatus;

            let next_high_regs = next.data.saved_registers.as_ptr();
            info!("next cleanup: {}", next.data.saved_registers[14]);
            Some((current_high_regs as u32, next_high_regs as u32))
        }) {
            break res;
        }
    };

    info!("result: {:?}-{:?}", current_high_regs, next_high_regs);
    (current_high_regs, next_high_regs)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn debug() {
    debug!("debug function called");
}

global_asm!(r#"
    .section .text         // FIXME: is this right ?

    .globl return_from_interrupt
    .align 4
    return_from_interrupt:
        la t0, {_CURRENT_CTX_PTR}
        lw t0, 0(t0)

        // if t0 is null, no need to save
        beqz    t0, restore
        // save registers
        sw s0, 0(t0)
        sw s1, 4(t0)
        sw s2, 8(t0)
        sw s3, 12(t0)
        sw s4, 16(t0)
        sw s5, 20(t0)
        sw s6, 24(t0)
        sw s7, 28(t0)
        sw s8, 32(t0)
        sw s9, 36(t0)
        sw s10, 40(t0)
        sw s11, 44(t0)
        sw sp, 48(t0)
        sw a0, 52(t0)
        sw ra, 56(t0)

        csrr t2, mepc


        sw t2, 60(t0)


    restore:
        // call {debug}
        la t1, {_NEXT_CTX_PTR}
        lw t0, 0(t1)

        sw x0, 0(t1)

        // load registers
        lw s0, 0(t0)
        lw s1, 4(t0)
        lw s2, 8(t0)
        lw s3, 12(t0)
        lw s4, 16(t0)
        lw s5, 20(t0)
        lw s6, 24(t0)
        lw s7, 28(t0)
        lw s8, 32(t0)
        lw s9, 36(t0)
        lw s10, 40(t0)
        lw s11, 44(t0)

        lw sp, 48(t0)
        lw a0, 52(t0)
        lw ra, 56(t0)
        lw t1, 64(t0)
        csrw mstatus, t1
        lw t2, 60(t0)
        csrw mepc,t2

        // set mpie an mie
        // csrr t0, mstatus
        // ori t0, t0, 0x88
        // csrw mstatus, t0
        // call {debug}


        mret
        // call {debug}

    "#,
    debug = sym debug,
    _CURRENT_CTX_PTR = sym _CURRENT_CTX_PTR,
    _NEXT_CTX_PTR = sym _NEXT_CTX_PTR,
);
