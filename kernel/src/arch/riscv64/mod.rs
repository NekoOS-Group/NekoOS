pub mod config;
pub mod mm;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));