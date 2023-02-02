#![allow(unused)]

mod lang;
pub mod heap;
pub mod logging;

pub use crate::arch::config::*;

// global
pub const KERNEL_STACK_SIZE: usize = 0x2000;
pub const KERNEL_HEAP_SIZE:  usize = 0x800000;
pub const USER_STACK_SIZE:   usize = 0x100000;

pub const TICKS_PER_SEC: usize = 100;

pub const PAGE_SIZE: usize = 4096;

pub const FDT_MAX_SIZE: usize = 0x2000;

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
    pub fn sbss_with_stack();
}