pub struct Entity {

}

impl super::ScheEntity for Entity {
    fn get_priority(&self) -> usize {
        todo!()
    }
}

pub struct CompletelyFairScheduler<E> 
    where E: super::ScheEntity
{
    qwq: E
}

impl<E> super::Scheduler<E> for CompletelyFairScheduler<E> 
    where E: super::ScheEntity
{
    fn schedule(&mut self) -> Option<E> {
        todo!()
    }

    fn push(&mut self, entity: E) {
        todo!()
    }
}