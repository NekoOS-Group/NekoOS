#![allow(unused)]
use core::arch::asm;

pub fn get_ra() -> usize {
    let ret: usize;
    unsafe { asm!("mv {0}, ra", out(reg)ret) }
    ret
}

pub fn get_sp() -> usize {
    let ret: usize;
    unsafe { asm!("mv {0}, sp", out(reg)ret) }
    ret
}

pub fn get_fp() -> usize {
    let ret: usize;
    unsafe { asm!("mv {0}, fp", out(reg)ret) }
    ret
}

pub fn get_gp() -> usize {
    let ret: usize;
    unsafe { asm!("mv {0}, gp", out(reg)ret) }
    ret
}

pub fn get_tp() -> usize {
    let ret: usize;
    unsafe { asm!("mv {0}, tp", out(reg)ret) }
    ret
}