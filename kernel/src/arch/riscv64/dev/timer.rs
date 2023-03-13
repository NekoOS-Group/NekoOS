pub use super::super::sbi::set_timer;

pub const CLOCK_PER_SEC: u64 = 100000000;

pub fn get_clock() -> u64 {
    riscv::register::time::read() as u64
}