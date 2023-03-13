use alloc::collections::LinkedList;
use bitmaps::Bitmap;

pub struct StaticPriorityScheduler<E> 
    where E: super::ScheEntity
{
    bitmap: Bitmap<256>,
    entity_lists: LinkedList<E>
}

impl<E> super::Scheduler<E> for StaticPriorityScheduler<E> 
    where E: super::ScheEntity
{
    fn schedule(&mut self) -> Option<E> {
        self.bitmap.last_index().map( |level| { 
            if self.entity_lists.len() == 1
                { self.bitmap.set(level, false); }
            self.entity_lists.pop_front().unwrap() 
        } )
    }

    fn push(&mut self, mut entity: E) {
        self.bitmap.set(entity.get_priority(), true);
        self.entity_lists.push_back(entity);
    }
}

