use crate::task::task_scheduler;

fn idle() -> ! {
    loop {
        task_scheduler::schedule()
    }
}

pub fn init() {

}