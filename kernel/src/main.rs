#![no_std]
#![no_main]
#![allow(unused)]

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

#[cfg(target_arch="riscv32")]
#[path="arch/riscv32/mod.rs"]
mod arch;

#[cfg(target_arch="riscv64")]
#[path="arch/riscv64/mod.rs"]
mod arch;

// Default architecture module if no specific architecture is targeted
#[cfg(not(any(target_arch="riscv32", target_arch="riscv64")))]
#[path="arch/riscv64/mod.rs"]
mod arch;

#[allow(unused)]
#[cfg(debug_assertions)]
mod debug;

// Entry point from arch/riscv64/entry.asm after paging is enabled.
#[unsafe(no_mangle)]
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

    // Boot sequence: memory -> timer -> traps -> tasks.
    mm::init(&fdt.memory());

    dev::timer::init();
    
    trap::init();
    trap::enable_timer_interrupt();
    dev::timer::set_next_trigger();
    trap::enable_trap();

    task::init();

    dev::cpu::shutdown()
}
