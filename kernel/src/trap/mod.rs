
mod context;
mod syscall;

use context::TrapContext;

use crate::println;

use riscv::{register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec, sstatus
} };

core::arch::global_asm!(include_str!("trap_entry.S"));

pub fn init() {
    extern "C" {
        fn __traps_user();
        fn __traps_sys();
    }
    unsafe {
        stvec::write(__traps_sys as usize, TrapMode::Direct);
    }
    crate::println!("[Neko] trap inited.");
}

pub fn init_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
    println!("[Neko] timer interrupt inited.");
}

pub fn enable_trap() {
    unsafe {
        sstatus::set_sie();
    }
    info!("trap enabled.");
}

#[allow(unused)]
pub fn disable_trap() {
    unsafe {
        sstatus::clear_sie();
    }
    info!("trap disabled.");
}

#[no_mangle]
pub fn trap_handler(context : &mut TrapContext ) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            context.sepc += 4;
            context.x[10] = syscall::syscall(
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
                scause.cause(),
                stval,
                context.sepc
            );
        }
    }
    context
}

#[no_mangle]
pub fn sys_trap_handler() -> () {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            crate::dev::timer::set_next_trigger();
        }
        _ => { 
            panic!(
                "Unsupported system trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval,
            );
        }
    };
}