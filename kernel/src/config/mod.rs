#![allow(unused)]

mod lang;

pub use crate::arch::config::*;

// global
pub const PAGE_SIZE: usize = 0x1000;

pub const KERNEL_STACK_SIZE: usize = 0x4000;
pub const KERNEL_HEAP_SIZE:  usize = 0x800000;

pub const TICKS_PER_SEC: usize = 100;

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