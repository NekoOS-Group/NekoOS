#![no_std]
#![no_main]

#![feature(panic_info_message)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate alloc;

mod config;
mod dev;
mod fs;
mod mm;
mod schedule;
mod trap;

#[cfg(riscv64)]
#[path="arch/riscv64/mod.rs"]
mod arch;

#[cfg(riscv32)]
#[path="arch/riscv32/mod.rs"]
mod arch;

#[allow(unused)]
#[cfg(debug_assertions)]
mod debug;

#[no_mangle]
fn start(hartid: usize, dtb: usize) -> ! {
    dev::timer::init();
    dev::console::init();

    println!( "[Neko] Nya~ from hart{} dtb @ {:#x}", hartid, dtb );

    let fdt = dev::fdt::get_fdt(dtb);

    mm::init(&fdt.memory());

    trap::init();
    trap::init_timer_interrupt();
    trap::enable_trap();

    dev::timer::set_next_trigger();
    
    dev::cpu::shutdown()
}
