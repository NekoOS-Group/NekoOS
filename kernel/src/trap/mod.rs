mod syscall;

pub use syscall::syscall;

pub fn init() {
    crate::arch::trap::init();
    crate::println!("[Neko] trap inited.");
}

pub fn init_timer_interrupt() {
    crate::arch::trap::init_timer_interrupt();
    crate::println!("[Neko] timer interrupt inited.");
}

pub fn enable_trap() {
    crate::arch::trap::enable_trap();
    info!("trap enabled.");
}

#[allow(unused)]
pub fn disable_trap() {
    crate::arch::trap::disable_trap();
    info!("trap disabled.");
}