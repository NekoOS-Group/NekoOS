#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod config;
mod sbi;
mod dev;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

fn bss_init() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize)
        .for_each(
            |a| { unsafe { (a as *mut u8).write_volatile(0) } } 
        );
}
   

#[no_mangle]
fn start() -> ! {
    bss_init();
    panic!("qwq");
}