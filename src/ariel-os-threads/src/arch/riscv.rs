#![expect(unsafe_code)]

use crate::{Arch, SCHEDULER, Thread, cleanup};
use ariel_os_debug::log::debug;
#[cfg(context = "esp32c6")]
use esp_hal::peripherals::INTPRI as SYSTEM;
#[cfg(context = "esp32c3")]
use esp_hal::peripherals::SYSTEM;
use esp_hal::{
    interrupt::{self, TrapFrame},
    peripherals::Interrupt,
    riscv,
    system::Cpu as EspHalCpu,
};

pub struct Cpu;

impl Arch for Cpu {
    type ThreadData = TrapFrame;
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
        thread.data.a0 = arg.unwrap_or_default();
        thread.data.ra = cleanup as usize;
        // thread.data.pc = func as usize;

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
        let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::min());
        debug!("e : {:?}", e);
        debug!("interrupt enabled");
    }

    fn wfi() {
        riscv::asm::wfi();
    }
}

const fn default_trap_frame() -> TrapFrame {
    TrapFrame {
        ra: 0,
        t0: 0,
        t1: 0,
        t2: 0,
        t3: 0,
        t4: 0,
        t5: 0,
        t6: 0,
        a0: 0,
        a1: 0,
        a2: 0,
        a3: 0,
        a4: 0,
        a5: 0,
        a6: 0,
        a7: 0,
    }
}

/// Copies the register state from `src` to `dst`.
///
/// It copies state from the `TrapFrame` except for CSR registers
/// `mstatus`, `mcause` and `mtval`.
fn copy_registers(src: &TrapFrame, dst: &mut TrapFrame) {
    debug!("Copying registers ");
    debug!("Copying registers src : {:?} ", src);
    debug!("Copying registers dst:  {:?}", dst);
    dst.ra = src.ra;
    dst.t0 = src.t0;
    dst.t1 = src.t1;
    dst.t2 = src.t2;
    dst.t3 = src.t3;
    dst.t4 = src.t4;
    dst.t5 = src.t5;
    dst.t6 = src.t6;
    dst.a0 = src.a0;
    dst.a1 = src.a1;
    dst.a2 = src.a2;
    dst.a3 = src.a3;
    dst.a4 = src.a4;
    dst.a5 = src.a5;
    dst.a6 = src.a6;
    dst.a7 = src.a7;
    debug!("Registers copied");
}

/// Handler for software interrupt 0, which we use for context switching.
// SAFETY: symbol required by `esp-pacs`.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn FROM_CPU_INTR0() {
    debug!("interrupt !");
    unsafe {
        // clear FROM_CPU_INTR0
        (&*SYSTEM::PTR)
            .cpu_intr_from_cpu(0)
            .modify(|_, w| w.cpu_intr().clear_bit());

        sched()
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
unsafe fn sched() {
    loop {
        debug!("sched loop");
        if SCHEDULER.with_mut(|mut scheduler| {
            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            debug!("get_next_tid");

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    Cpu::wfi();
                    return false;
                }
            };

            debug!("current_tid");

            if let Some(current_tid) = scheduler.current_tid() {
                if next_tid == current_tid {
                    return true;
                }
                // TODO: save registers
                // copy_registers(
                //     trap_frame,
                //     &mut scheduler.threads[usize::from(current_tid)].data,
                // );
            }
            debug!("modifying current tid");

            *scheduler.current_tid_mut() = Some(next_tid);

            debug!("copying registers");

            // TODO(bump): restore registers
            // copy_registers(&scheduler.get_unchecked(next_tid).data, trap_frame);
            debug!("should return");

            true
        }) {
            debug!("break");
            break;
        }
    }
}
