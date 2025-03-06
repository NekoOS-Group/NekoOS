#![allow(unused)]

// arch base config
pub use crate::arch::config::*;

// global config
pub const PAGE_SIZE          : usize = 0x1000;

pub const KERNEL_STACK_SIZE  : usize = 0x4000;

pub const TICKS_PER_SEC      : u64   = 100;

// kernel symbol
unsafe extern "C" {
    pub fn skernel();
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss();
    pub fn bootstack();
    pub fn bootstacktop();
    pub fn bootheap();
    pub fn bootheapend();
    pub fn ebss();
    pub fn ekernel();
}