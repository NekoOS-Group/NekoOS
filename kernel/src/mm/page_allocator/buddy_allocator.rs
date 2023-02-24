use super::PageAllocator;
use crate::mm::page::Page;
use buddy_system_allocator::LockedFrameAllocator;

pub struct BuddyAllocator {
    inner: LockedFrameAllocator
}

impl PageAllocator for BuddyAllocator {
    fn new() -> Self {
        BuddyAllocator { inner: LockedFrameAllocator::new() }
    }
    fn add(&mut self, l: usize, r: usize) {
        self.inner.lock().insert(l..r);
    }
    fn alloc(&mut self) -> Option<Page> {
        if let Some(ppn) = self.inner.lock().alloc(1) {
            let p = Page::new(ppn);
            p.clear();
            Some(p)
        } else {
            None
        }
    }
    fn dealloc(&mut self, ppn: usize) {
        self.inner.lock().dealloc(ppn, 1)
    }
}