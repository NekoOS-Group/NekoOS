use alloc::vec::Vec;
use crate::mm::page::Page;

pub struct StackAllocator {
    l: usize,
    r: usize,
    stack :Vec<usize>
}

impl super::PageAllocator for StackAllocator {
    fn new(l: usize, r:usize) -> Self {
        StackAllocator { l, r, stack: Vec::new() }
    }
    fn alloc(&mut self) -> Option<Page> {
        if self.stack.len() > 0 {
            Some(Page::new(self.stack.pop().unwrap() ))
        } else if self.l != self.r {
            self.l += 1;
            Some(Page::new(self.l - 1))
        } else {
            None
        }
    }
    fn dealloc(&mut self, ppn: usize) {
        self.stack.push(ppn);
    }
}