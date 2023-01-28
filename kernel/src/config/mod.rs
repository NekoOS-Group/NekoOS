#![allow(unused)]

mod lang;
pub mod heap;
pub mod logging;

pub const TICKS_PER_SEC: usize = 100;

pub const KERNEL_STACK_SIZE: usize = 0x2000;
pub const KERNEL_HEAP_SIZE: usize = 0x300000;
pub const USER_STACK_SIZE: usize = 0x100000;

pub const PAGE_SIZE: usize = 4096;

pub const EARLY_MEMORY_END: usize = 0x88000000;

extern "C" {
    pub fn skernel();
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss();
    pub fn ebss();
    pub fn ekernel();
}