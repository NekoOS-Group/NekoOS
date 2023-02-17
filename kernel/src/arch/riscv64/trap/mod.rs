pub mod syscall;

mod context;

use context::TrapContext;
use riscv::register;

use riscv::register::{
    mtvec::TrapMode,
    scause::{Exception, Interrupt, Trap},
};

core::arch::global_asm!(include_str!("trap_entry.S"));

pub fn init() {
    extern "C" {
        fn __traps_user();
        fn __traps_sys();
    }
    unsafe {
        register::stvec::write(__traps_sys as usize, TrapMode::Direct);
    }
}

pub fn init_timer_interrupt() {
    unsafe {
        register::sie::set_utimer()
    }
}

pub fn enable_trap() {
    unsafe {
        register::sstatus::set_sie();
    }
}

#[allow(unused)]
pub fn disable_trap() {
    unsafe {
        register::sstatus::clear_sie();
    }
}

#[no_mangle]
pub fn trap_handler(context : &mut TrapContext) -> &mut TrapContext{
    let scause = register::scause::read();
    let stval = register::stval::read();
    match scause.cause() {
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
    let scause = register::scause::read();
    let stval = register::stval::read();
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