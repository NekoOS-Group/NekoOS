#![no_std]
#![no_main]

mod lang;
mod sbi;

use core::arch::global_asm;

use sbi::{sbi_putchar, sbi_shutdown};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn start() {
    sbi_putchar('c' as usize);
    sbi_shutdown();
}