mod syscall;

pub use syscall::syscall;

pub use crate::arch::trap::context::Context;

pub fn init() {
    crate::arch::trap::init();
    crate::println!("[Neko] trap inited.");
}

pub fn enable_timer_interrupt() {
    crate::arch::trap::enable_stimer_interrupt();
    info!("timer interrupt enabled.");
}

pub fn disable_timer_interrupt() {
    crate::arch::trap::disable_stimer_interrupt();
    info!("[Neko] timer interrupt disabled.");
}

pub fn enable_trap() {
    crate::arch::trap::enable_trap();
    info!("trap enabled.");
}

pub fn disable_trap() {
    crate::arch::trap::disable_trap();
    info!("trap disabled.");
}
