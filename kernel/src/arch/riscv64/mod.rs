#![allow(unused)]
pub mod config;
pub mod mm;
pub mod trap;
pub mod dev;
pub mod register;

mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));