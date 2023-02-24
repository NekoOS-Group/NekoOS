use crate::schedule::task_scheduler;

fn idle() -> ! {
    loop {
        task_scheduler::schedule()
    }
}

pub fn init() {

}