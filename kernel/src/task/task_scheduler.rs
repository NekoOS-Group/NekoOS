use crate::task::{Processor, Thread};
use crate::algorithm::schedule;

struct TaskScheduler {

}

impl schedule::SchePreemptor for Thread {
    
}

impl schedule::SchePriority for Thread {

}

impl schedule::ScheResource for Processor {

}

impl schedule::Scheduler<Processor, Thread> for TaskScheduler {
    
}

pub fn schedule() {
    
}