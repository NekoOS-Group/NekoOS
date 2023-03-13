mod completely_fair_scheduler;
mod static_priority_scheduler;

pub trait ScheEntity {
    fn get_priority(&self) -> usize;
}

pub trait Scheduler<E> 
    where E: ScheEntity
{
    fn schedule(&mut self) -> Option<E>;
    fn push(&mut self, entity: E) {

    }
}