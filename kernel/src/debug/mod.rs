use core::arch::asm;

pub fn get_sp() -> usize {
    let mut x: usize;
    unsafe {
        asm!( "mv {0}, sp", out(reg) x );
    }
    x
}