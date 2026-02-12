use alloc::vec::Vec;

use super::Allocator;

pub struct StackAllocator {
    // [l, r) is the contiguous free range; stack stores recycled pages.
    l: usize,
    r: usize,
    stack: Vec<usize>
}

impl Allocator for StackAllocator {
    fn new() -> Self {
        StackAllocator { l: 0, r: 0, stack: Vec::new() }
    }
    fn add(&mut self, l: usize, r:usize) {
        self.l = l;
        self.r = r;
    }
    fn alloc(&mut self) -> Option<usize> {
        if self.stack.len() > 0 {
            Some(self.stack.pop().unwrap())
        } else if self.l != self.r {
            self.l += 1;
            Some(self.l - 1)
        } else {
            None
        }
    }
    fn dealloc(&mut self, ppn: usize) {
        self.stack.push(ppn);
    }
}
