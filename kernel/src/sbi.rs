#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

pub fn timer(timer: usize) 
  { sbi_call(SBI_SET_TIMER, timer, 0, 0); }

pub fn console_putchar(c: usize) 
  { sbi_call( SBI_CONSOLE_PUTCHAR, c, 0, 0); }

pub fn console_getchar() -> usize 
  { sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0) }

pub fn clear_ipi()
  { sbi_call(SBI_CLEAR_IPI, 0, 0, 0); }

pub fn send_ipi(hart_mask: usize)
  { sbi_call(SBI_SEND_IPI, hart_mask, 0, 0); }

pub fn remote_fence_i(hart_mask : usize)
  { sbi_call(SBI_REMOTE_FENCE_I, hart_mask, 0, 0); }

pub fn remote_sfence_vma(hart_mask: usize, start: usize, size: usize)
  { sbi_call(SBI_REMOTE_SFENCE_VMA, hart_mask, 0, 0); }

pub fn remote_sfence_vma_asid(hart_mask: usize, start: usize, size: usize)
  { sbi_call(SBI_REMOTE_SFENCE_VMA_ASID, hart_mask, 0, 0); }

pub fn shutdown() -> ! 
  { sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!"); }