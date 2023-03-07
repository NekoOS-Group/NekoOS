use crate::println;

use super::Allocator;
use buddy_system_allocator::LockedFrameAllocator;

pub struct BuddyAllocator {
    inner: LockedFrameAllocator
}

impl Allocator for BuddyAllocator {
    fn new() -> Self {
        BuddyAllocator { inner: LockedFrameAllocator::new() }
    }
    fn add(&mut self, l: usize, r: usize) {
        self.inner.lock().insert(l..r);
    }
    fn alloc(&mut self) -> Option<usize> {
        self.inner.lock().alloc(1) 
    }
    fn dealloc(&mut self, ppn: usize) {
        self.inner.lock().dealloc(ppn, 1)
    }
}