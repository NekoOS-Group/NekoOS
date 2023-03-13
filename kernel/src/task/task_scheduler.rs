use crate::task::ThreadRef;
use crate::algorithm::scheduler;

struct TaskScheduler {

}

impl scheduler::ScheEntity for ThreadRef {
    fn get_priority(&self) -> usize {
        0
    }
}

impl scheduler::Scheduler<ThreadRef> for TaskScheduler {
    fn schedule(&mut self) -> Option<ThreadRef> {
        todo!()
    }
}

pub fn schedule() {
    
}