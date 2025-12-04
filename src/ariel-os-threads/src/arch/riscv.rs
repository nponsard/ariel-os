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
pub struct Cpu;

#[derive(Debug)]
pub struct ThreadData {}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = default_trap_frame();

    /// Triggers software interrupt for the context switch.
    fn schedule() {
        unsafe {
            (&*SYSTEM::PTR)
                .cpu_intr_from_cpu(0)
                .modify(|_, w| w.cpu_intr().set_bit());
        }
    }

    /// On RISC-V (ESP32), the stack doesn't need to be set up with any register values since
    /// they are restored from the stored [`TrapFrame`].
    fn setup_stack(thread: &mut Thread, stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let stack_start = stack.as_ptr() as usize;
        // 16 byte alignment.
        let stack_pos = (stack_start + stack.len()) & 0xFFFFFFE0;
        // Set up PC, SP, RA and first argument for function.
        // thread.data.sp = stack_pos;
        // thread.data.a0 = arg.unwrap_or_default();
        // thread.data.ra = cleanup as usize;
        // thread.data.pc = func as usize;

        // thread.stack_lowest = stack_start;
        // thread.stack_highest = stack_pos;

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
        let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::min());
        debug!("e : {:?}", e);
        debug!("interrupt enabled");
    }

    fn wfi() {
        riscv::asm::wfi();
    }
}

const fn default_trap_frame() -> ThreadData {
    ThreadData {}
}

/// Copies the register state from `src` to `dst`.
///
/// It copies state from the `TrapFrame` except for CSR registers
/// `mstatus`, `mcause` and `mtval`.
// fn copy_registers(src: &TrapFrame, dst: &mut TrapFrame) {
//     let (mstatus, mcause, mtval) = (dst.mstatus, dst.mcause, dst.mtval);
//     dst.clone_from(src);
//     dst.mstatus = mstatus;
//     dst.mcause = mcause;
//     dst.mtval = mtval;
// }

global_asm!(
    r#"

    .section .text          // FIXME: is this right ?
    .globl FROM_CPU_INTR0
    .align 4
    FROM_CPU_INTR0:
        call {sched}
        call {sched}
    "#,
    sched = sym sched
);

/// Handler for software interrupt 0, which we use for context switching.
// SAFETY: symbol required by `esp-pacs`.
// #[allow(non_snake_case)]
// #[unsafe(no_mangle)]
// fn FROM_CPU_INTR0() {
//     debug!("interrupt !");
//     unsafe {
//         // clear FROM_CPU_INTR0
//         (&*SYSTEM::PTR)
//             .cpu_intr_from_cpu(0)
//             .modify(|_, w| w.cpu_intr().clear_bit());

//         sched();
//     }
// }

/// Probes the runqueue for the next thread and switches context if needed.
///
/// # Safety
///
/// This method might switch the current register state that is saved in the
/// `trap_frame`.
/// It should only be called from inside the trap handler that is responsible for
/// context switching.
unsafe extern "C" fn sched() -> u64 {
    info!("sched !");
    0
    // loop {
    //     if SCHEDULER.with_mut(|mut scheduler| {
    //         #[cfg(feature = "multi-core")]
    //         scheduler.add_current_thread_to_rq();

    //         let next_tid = match scheduler.get_next_tid() {
    //             Some(tid) => tid,
    //             None => {
    //                 Cpu::wfi();
    //                 return false;
    //             }
    //         };

    //         if let Some(current_tid) = scheduler.current_tid() {
    //             if next_tid == current_tid {
    //                 return true;
    //             }
    //             copy_registers(
    //                 trap_frame,
    //                 &mut scheduler.threads[usize::from(current_tid)].data,
    //             );
    //         }
    //         *scheduler.current_tid_mut() = Some(next_tid);

    //         copy_registers(&scheduler.get_unchecked(next_tid).data, trap_frame);
    //         true
    //     }) {
    //         break;
    //     }
    // }
}
