#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate alloc;

mod config;
mod debug;

mod sbi;
mod dev;
mod trap;
mod fs;
mod mm;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

fn bss_init() {
    use config::{sbss, ebss};
    (sbss as usize..ebss as usize).for_each(
        |a| { unsafe { (a as *mut u8).write_volatile(0) } } 
    );
}
   

#[no_mangle]
fn start() -> ! {
    bss_init();
    config::logging::init();
    config::heap::init();

    println!( "[Neko] Hello World!" );
    
    mm::init();

    trap::init();
    trap::init_timer_interrupt();
    trap::enable_trap();

    dev::timer::set_next_trigger();
    
    sbi::shutdown()
}
