#![expect(unsafe_code)]

use crate::arch::riscv::riscv::register;
use ariel_os_debug::log::{debug, error, info, trace};
use core::arch::global_asm;
use core::ptr;
use esp_hal::{
    interrupt::{self, InterruptHandler, Priority, software::SoftwareInterrupt},
    peripherals::Interrupt,
    riscv,
    system::Cpu as EspHalCpu,
};
use portable_atomic::Ordering;

use crate::{Arch, SCHEDULER, Thread, cleanup, schedule};

pub struct Cpu;

unsafe extern "C" {
    fn sys_switch();
}

static _CURRENT_CTX_PTR: portable_atomic::AtomicPtr<ThreadData> =
    portable_atomic::AtomicPtr::new(core::ptr::null_mut());

static _NEXT_CTX_PTR: portable_atomic::AtomicPtr<ThreadData> =
    portable_atomic::AtomicPtr::new(core::ptr::null_mut());

#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadData {
    ra: usize,
    sp: usize,
    gp: usize,
    tp: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    s0: usize,
    s1: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    mstatus: usize,
    mepc: usize,
}
/// aaaaa
pub fn interrupt_status() {
    let mstatus_st = esp_hal::riscv::register::mstatus::read();
    trace!(
        "mstatus.mie {}, mstatus.mpie {}",
        mstatus_st.mie(),
        mstatus_st.mpie()
    );
}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = default_trap_frame();

    /// Triggers software interrupt for the context switch.
    fn schedule() {
        // let mstatus_st = esp_hal::riscv::register::mstatus::read();
        // trace!(
        //     "schedule called, mstatus.mie {}, mstatus.mpie {}",
        //     mstatus_st.mie(),
        //     mstatus_st.mpie()
        // );

        // SAFETY: `steal().raise()` is safe on an initialized software interrupt
        unsafe { SoftwareInterrupt::<0>::steal().raise() }
    }

    fn setup_stack(thread: &mut Thread, stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let stack_start = stack.as_ptr() as usize;
        // 16 byte alignment.
        let stack_pos = (stack_start + stack.len()) & 0xFFFFFFE0;
        // Set up PC, SP, RA and first argument for function.
        // sp
        thread.data.sp = stack_pos;
        // a0
        thread.data.a0 = arg.unwrap_or_default();

        // ra
        thread.data.ra = cleanup as *const () as usize;
        // pc
        thread.data.mepc = func as usize;

        thread.stack_lowest = stack_start;
        thread.stack_highest = stack_pos;

        // Safety: This is the place to initialize stack painting.
        unsafe { thread.stack_paint_init(stack_pos) };
    }

    /// Enable and trigger the appropriate software interrupt.
    fn start_threading() {
        // unsafe {
        //     core::arch::asm!("ebreak");
        // }

        // TODO: check safety
        unsafe {
            SoftwareInterrupt::<0>::steal()
                .set_interrupt_handler(InterruptHandler::new_not_nested(sched, Priority::Priority1));
        }

        schedule();
        debug!("interrupt enabled");
    }

    fn wfi() {
        riscv::asm::wfi();
    }
}

const fn default_trap_frame() -> ThreadData {
    ThreadData {
        ra: 0,
        sp: 0,
        gp: 0,
        tp: 0,
        t0: 0,
        t1: 0,
        t2: 0,
        s0: 0,
        s1: 0,
        a0: 0,
        a1: 0,
        a2: 0,
        a3: 0,
        a4: 0,
        a5: 0,
        a6: 0,
        a7: 0,
        s2: 0,
        s3: 0,
        s4: 0,
        s5: 0,
        s6: 0,
        s7: 0,
        s8: 0,
        s9: 0,
        s10: 0,
        s11: 0,
        t3: 0,
        t4: 0,
        t5: 0,
        t6: 0,
        mstatus: 0x80, // MPIE set
        mepc: 0,
    }
}

global_asm!(
    r#"

    .section .trap, "ax"          // FIXME: is this right ?
    .globl sys_switch
    .align 4
    sys_switch:


        // save some registers for scratch space
        addi sp, sp, -0x10
        sw a0, 0(sp)
        sw a1, 4(sp)
        sw t0, 8(sp)


        la a0, {_CURRENT_CTX_PTR}
        lw a0, 0(a0)

        // if a0 is null, no need to save
        beqz    a0, restore


        // save registers

        // mepc is set by the "caller"

        //ra
        sw ra, 0*4(a0)

        // gp
        sw gp, 2*4(a0)

        // tp
        sw tp, 3*4(a0)

        // t0
        lw t0, 8(sp)
        sw t0, 4*4(a0)

        // t1
        sw t1, 5*4(a0)

        // t2
        sw t2, 6*4(a0)

        sw s0, 7*4(a0)
        sw s1, 8*4(a0)

        // a0
        lw t0, 0(sp)
        sw t0, 9*4(a0)

        // a1
        lw t0, 4(sp)
        sw t0, 10*4(a0)

        // a2
        sw a2, 11*4(a0)

        // a3
        sw a3, 12*4(a0)

        // a4
        sw a4, 13*4(a0)

        // a5
        sw a5, 14*4(a0)

        // a6
        sw a6, 15*4(a0)

        // a7
        sw a7, 16*4(a0)

        sw s2, 17*4(a0)
        sw s3, 18*4(a0)
        sw s4, 19*4(a0)
        sw s5, 20*4(a0)
        sw s6, 21*4(a0)
        sw s7, 22*4(a0)
        sw s8, 23*4(a0)
        sw s9, 24*4(a0)
        sw s10, 25*4(a0)
        sw s11, 26*4(a0)

        // t3
        sw t3, 27*4(a0)

        // t4
        sw t4, 28*4(a0)

        // t5
        sw t5, 29*4(a0)

        // t6
        sw t6, 30*4(a0)

        addi t0, sp, 0x10
        sw t0, 1*4(a0)

    restore:

        la a1, {_NEXT_CTX_PTR}
        lw a1, 0(a1)

        // restore mepc and mstatus
        lw t0, 31*4(a1)
        csrw mstatus, t0
        lw t0, 32*4(a1)
        csrw mepc, t0

        // load registers
        lw ra, 0*4(a1)
        lw sp, 1*4(a1)
        lw gp, 2*4(a1)
        lw tp, 3*4(a1)
        lw t0, 4*4(a1)
        lw t1, 5*4(a1)
        lw t2, 6*4(a1)
        lw s0, 7*4(a1)
        lw s1, 8*4(a1)
        lw a0, 9*4(a1)
        lw a2, 11*4(a1)
        lw a3, 12*4(a1)
        lw a4, 13*4(a1)
        lw a5, 14*4(a1)
        lw a6, 15*4(a1)
        lw a7, 16*4(a1)
        lw s2, 17*4(a1)
        lw s3, 18*4(a1)
        lw s4, 19*4(a1)
        lw s5, 20*4(a1)
        lw s6, 21*4(a1)
        lw s7, 22*4(a1)
        lw s8, 23*4(a1)
        lw s9, 24*4(a1)
        lw s10, 25*4(a1)
        lw s11, 26*4(a1)
        lw t3, 27*4(a1)
        lw t4, 28*4(a1)
        lw t5, 29*4(a1)
        lw t6, 30*4(a1)

        lw a1, 10*4(a1)

        mret

        "#,
        _CURRENT_CTX_PTR = sym _CURRENT_CTX_PTR,
        _NEXT_CTX_PTR = sym _NEXT_CTX_PTR,

);

/// Probes the runqueue for the next thread and switches context if needed.
// #[esp_hal::ram]
extern "C" fn sched() {
    unsafe { SoftwareInterrupt::<0>::steal().reset() }
    // unsafe {
    //     esp_hal::peripherals::PLIC_MX::regs()
    //         .mxint_thresh()
    //         .write(|w| unsafe { w.cpu_mxint_thresh().bits(4) });
    // }
    let mstatus_st = esp_hal::riscv::register::mstatus::read();
    let mstatus = mstatus_st.bits();

    // trace!("sched mstatus {:#x}", mstatus);

    // clear FROM_CPU_INTR0
    // SAFETY: `steal().reset()` is safe on an initialized software interrupt

    let (current_high_regs, next_high_regs) = loop {
        if let Some(res) = SCHEDULER.with_mut(|mut scheduler| {
            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    return None;
                }
            };

            let mut current_high_regs = core::ptr::null_mut();

            if let Some(current_tid_ref) = scheduler.current_tid_mut() {
                // if next_tid == *current_tid_ref {
                //     return Some((ptr::null_mut(), ptr::null_mut()));
                // }
                let current_tid = *current_tid_ref;
                *current_tid_ref = next_tid;
                let current = scheduler.get_unchecked_mut(current_tid);
                current_high_regs = &raw mut current.data;
            } else {
                *scheduler.current_tid_mut() = Some(next_tid);
            }
            let next = scheduler.get_unchecked_mut(next_tid);
            // next.data.mstatus = mstatus;

            let next_high_regs = &raw mut next.data;
            // trace!("next mepc: {:#x}", next.data.mepc);
            Some((current_high_regs, next_high_regs))
        }) {
            break res;
        } else {
            unreachable!("idle threads should be enabled")
        }
    };

    // debug!(
    //     "Scheduler result: {:?}-{:?}",
    //     current_high_regs, next_high_regs
    // );

    // let mepc = esp_hal::riscv::register::mepc::read();
    // trace!("sched mepc: {:#x}", mepc);

    // interrupt_status();

    // The caller expects these two pointers in a0 and a1:
    // a0 = &current.data.high_regs (or 0)
    // a1 = &next.data.high_regs

    // trace!("sched end sp {:#x}", sp());
    // trace!("sched end ra: {:#x}", ra());
    // trace!("coming sp : {:#x}", coming_sp);

    // if next_high_regs == 0 {
    //     for i in 0..20 {
    //         unsafe {
    //             let offset = i * 4;

    //             trace!(
    //                 "stack +{} ({:#x}): {:#x}",
    //                 offset,
    //                 coming_sp + offset,
    //                 *((coming_sp + offset) as *const u32)
    //             );
    //         }
    //     }
    // }

    // unsafe {
    //     esp_hal::peripherals::PLIC_MX::regs()
    //         .mxint_thresh()
    //         .write(|w| unsafe { w.cpu_mxint_thresh().bits(0) });
    // }

    if !current_high_regs.is_null() {
        unsafe {
            (*current_high_regs).mepc = register::mepc::read();
        }
    }

    let mstatus = register::mstatus::read().bits();
    unsafe {
        (*next_high_regs).mstatus = mstatus;
    }

    // return to the same task
    if next_high_regs.is_null() {
        return;
    }

    _CURRENT_CTX_PTR.store(current_high_regs, Ordering::SeqCst);
    _NEXT_CTX_PTR.store(next_high_regs, Ordering::SeqCst);

    unsafe {
        // set MPIE in MSTATUS to 0 to disable interrupts while task switching
        register::mstatus::write(register::mstatus::Mstatus::from_bits(mstatus & !(1 << 7)));

        // load address of sys_switch into MEPC - will run after all registers are restored
        register::mepc::write(sys_switch as *const () as usize);
    }

    // let mepc = esp_hal::riscv::register::mepc::read();
    // trace!("sched end mepc: {:#x}", mepc);

    // (current_high_regs as u64) | (next_high_regs as u64) << 32
}

/// Returns the current `SP` register value.
pub(crate) fn sp() -> usize {
    let sp: usize;
    // Safety: reading SP is safe
    unsafe {
        core::arch::asm!(
            "mv {}, sp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        )
    };
    sp
}

/// Returns the current `ra` register value.
pub(crate) fn ra() -> usize {
    let ra: usize;
    // Safety: reading SP is safe
    unsafe {
        core::arch::asm!(
            "mv {}, ra",
            out(reg) ra,
            options(nomem, nostack, preserves_flags)
        )
    };
    ra
}

// #[esp_hal::ram]
unsafe extern "C" fn log_sp_zero(sp: u32) {
    error!("sp is zero")
}
// #[esp_hal::ram]
unsafe extern "C" fn log_mepc_zero() {
    error!("mepc is zero")
}
