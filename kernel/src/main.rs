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

#[cfg(riscv64)]
#[path="arch/riscv64/mod.rs"]
mod arch;

#[cfg(riscv32)]
#[path="arch/riscv32/mod.rs"]
mod arch;

fn bss_init() {
    use config::{sbss, ebss};
    (sbss as usize..ebss as usize).for_each(
        |a| { unsafe { (a as *mut u8).write_volatile(0) } } 
    );
}
   

#[no_mangle]
fn start(hartid: usize, dtb: usize) -> ! {
    bss_init();

    println!( "[Neko] Nya~ from hart{} dtb @ {:#x}", hartid, dtb );

    config::logging::init();
    config::heap::init();
    config::heap::test();
    
    let fdt = dev::fdt::get_fdt( 
        unsafe {core::slice::from_raw_parts(dtb as *const u8, config::FDT_MAX_SIZE)} 
    );

    mm::init(&fdt.memory());

    trap::init();
    trap::init_timer_interrupt();
    trap::enable_trap();

    dev::timer::set_next_trigger();
    
    sbi::shutdown()
}
