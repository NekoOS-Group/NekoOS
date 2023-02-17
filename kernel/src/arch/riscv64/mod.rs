#![allow(unused)]
pub mod config;
pub mod register;
pub mod mm;
pub mod io;
pub mod cpu;
pub mod timer;
pub mod trap;

mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));