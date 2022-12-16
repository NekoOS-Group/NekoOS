#![allow(unused)]
use crate::config::TICKS_PER_SEC;
use crate::println;
use crate::sbi::set_timer;

const CLOCK_FREQUENCE: usize = 100000000;

//pub static mut ticks : usize = 0;

pub fn get_time() -> usize {
    riscv::register::time::read()
}

pub fn set_next_trigger() {
    let x: usize = get_time() + CLOCK_FREQUENCE / TICKS_PER_SEC;
    set_timer(x);
    println!("[debug] timer: next trigger is at {}", x);
}