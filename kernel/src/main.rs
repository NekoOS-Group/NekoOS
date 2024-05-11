#![no_std]
#![no_main]
#![allow(unused)]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate alloc;

mod lang;

mod algorithm;
mod config;
mod dev;
mod fs;
mod mm;
mod task;
mod trap;

#[cfg(target_arch="riscv64")]
#[path="arch/riscv64/mod.rs"]
mod arch;

#[cfg(target_arch="riscv32")]
#[path="arch/riscv32/mod.rs"]
mod arch;


#[allow(unused)]
#[cfg(debug_assertions)]
mod debug;

#[no_mangle]
fn start(hartid: usize, dtb: usize) -> ! {
    dev::console::init();

    println!( "[Neko] Welcome to NekoOS, Nya~ ");
    println!( 
        "[Neko] hart{} boot with {{ stack: [{:#x}, {:#x}) }}", 
        hartid,
        config::bootstack as usize + hartid * 4096 * 4, 
        config::bootstack as usize + (hartid + 1) * 4096 * 4,
    );

    let fdt = dev::fdt::get_fdt(dtb);

    mm::init(&fdt.memory());

    dev::timer::init();
    
    trap::init();
    trap::enable_trap();
    trap::enable_timer_interrupt();

    task::init();

    dev::timer::set_next_trigger();

    dev::cpu::shutdown()
}
