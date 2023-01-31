use super::PageAllocator;
use crate::mm::page::Page;
use buddy_system_allocator::FrameAllocator;

pub struct BuddyAllocator {
    inner: FrameAllocator
}

impl PageAllocator for BuddyAllocator {
    fn new() -> Self {
        BuddyAllocator { inner: FrameAllocator::new() }
    }
    fn add(&mut self, l: usize, r: usize) {
        self.inner.add_frame(l, r);
    }
    fn alloc(&mut self) -> Option<Page> {
        if let Some(ppn) = self.inner.alloc(1) {
            let p = Page::new(ppn);
            p.clear();
            Some(p)
        } else {
            None
        }
    }
    fn dealloc(&mut self, ppn: usize) {
        self.inner.dealloc(ppn, 1)
    }
}