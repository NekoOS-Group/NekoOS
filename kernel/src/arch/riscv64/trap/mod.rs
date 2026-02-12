pub mod syscall;
pub mod context;

use context::Context;
use riscv::interrupt::supervisor::{Exception, Interrupt};
use riscv::interrupt::Trap;
use riscv::register;
use riscv::register::mtvec::TrapMode;

use crate::{println, dev};

core::arch::global_asm!(include_str!("trap_entry.asm"));

pub fn init() {
    unsafe extern "C" {
        unsafe fn __traps_user();
        unsafe fn __traps_sys();
    }
    unsafe {
        let stvec = register::stvec::Stvec::new(__traps_sys as usize, TrapMode::Direct);
        register::stvec::write(stvec);
        // Start from a clean SIE state so enabling global interrupts
        // won't trigger unexpected traps.
        register::sie::clear_ssoft();
        register::sie::clear_sext();
        register::sie::clear_stimer();
        register::sip::clear_ssoft();
    }
}

pub fn enable_stimer_interrupt() 
    { unsafe { register::sie::set_stimer() } }

pub fn disable_stimer_interrupt() 
    { unsafe { register::sie::clear_stimer() } }

pub fn enable_trap() 
    { unsafe { register::sstatus::set_sie(); } }

pub fn disable_trap() 
    { unsafe { register::sstatus::clear_sie(); } }

#[unsafe(no_mangle)]
pub fn trap_handler(context : &mut Context) -> &mut Context{
    let scause = register::scause::read();
    let stval   = register::stval::read();
    let trap: Trap<Interrupt, Exception> = scause.cause().try_into().unwrap();
    match trap {
        Trap::Exception(Exception::UserEnvCall) => {
            context.sepc += 4;
            context.x[10] = crate::trap::syscall(
                context.x[17], 
                [context.x[10], context.x[11], context.x[12]]
            ) as usize;
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            crate::dev::timer::set_next_trigger();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!, sepc = {:#x}",
                trap,
                stval,
                context.sepc
            );
        }
    }
    context
}

#[unsafe(no_mangle)]
pub fn sys_trap_handler() -> () {
    debug!( "Trap recived at {:?}", dev::timer::get_time() );
    let scause = register::scause::read();
    let stval   = register::stval::read();
    let trap: Trap<Interrupt, Exception> = scause.cause().try_into().unwrap();
    match trap {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            crate::dev::timer::set_next_trigger();
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            unsafe { register::sip::clear_ssoft(); }
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            unsafe { register::sie::clear_sext(); }
        }
        _ => { 
            panic!(
                "Unsupported system trap {:?}, stval = {:#x}!",
                trap,
                stval,
            );
        }
    };
}
