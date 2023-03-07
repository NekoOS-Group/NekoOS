use crate::config::TICKS_PER_SEC;
pub use crate::arch::timer::set_timer;
pub use crate::arch::timer::get_clock;
pub use crate::arch::timer::CLOCK_PER_SEC;
const NANO_PER_SEC: u64 = 1_000_000_000;

use naive_timer::Timer;

pub static mut TICKS: usize = 0;
pub static mut TIMER: Option<Timer> = None; 

pub fn get_time() -> core::time::Duration {
    core::time::Duration::from_nanos( get_clock() * NANO_PER_SEC / CLOCK_PER_SEC )
}

pub fn set_next_trigger() {
    let x = get_clock() + CLOCK_PER_SEC / TICKS_PER_SEC;
    set_timer(x as usize);
    debug!("timer: next trigger is at {}", x);
}

pub fn init() {
    unsafe{ TIMER = Some(Timer::default()) };
}

pub fn schedule(
    time: core::time::Duration, 
    closure: impl FnOnce(core::time::Duration) + Send + Sync + 'static
) {
    unsafe {
        if let Some(inner) = &mut TIMER {
            inner.add(time, closure);
        }
    }
}

pub fn alarm() {
    unsafe {
        if let Some(inner) = &mut TIMER {
            inner.expire( get_time() );
        }
    }
}