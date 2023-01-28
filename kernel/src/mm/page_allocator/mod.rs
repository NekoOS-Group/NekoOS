use alloc::boxed::Box;
use super::page::Page;

mod stack_allocator;

trait PageAllocator {
    fn new(l: usize, r: usize) -> Self;
    fn alloc(&mut self) -> Option<Page>;
    fn dealloc(&mut self, ppn: usize);
}

type PageAllocatorImpl = stack_allocator::StackAllocator;

pub static mut GLOABAL_ALLOCATOR: Option<Box<PageAllocatorImpl>> = None;

pub fn alloc() -> Option<Page> {
    unsafe{ if let Some(ref mut inner) = GLOABAL_ALLOCATOR {
        inner.alloc()
    } else { None } }
}

pub fn dealloc(ppn: usize) {
    unsafe{ if let Some(ref mut inner) = GLOABAL_ALLOCATOR {
        inner.dealloc(ppn);
    } else {} }}

pub fn init() {
    use crate::config::{ekernel, EARLY_MEMORY_END, PAGE_SIZE};
    info!( "assigned user region {:#x}-{:#x}({} pages in tot)", 
        ekernel as usize, 
        EARLY_MEMORY_END as usize, 
        (EARLY_MEMORY_END - ekernel as usize) / PAGE_SIZE
    );
    unsafe {
        GLOABAL_ALLOCATOR = Some(Box::new(PageAllocatorImpl::new(
            ekernel as usize / PAGE_SIZE,
            EARLY_MEMORY_END / PAGE_SIZE
        )))
    }
    crate::println!("[Neko] page allocator inited.");
}

#[allow(unused)]
pub fn test() {
    use alloc::vec::Vec;
    let mut v: Vec<Page> = Vec::new();
    for _ in 0..5 {
        let frame = alloc().unwrap();
        v.push(frame);
    }
    v.clear();
    for _ in 0..5 {
        let frame = alloc().unwrap();
        v.push(frame);
    }
    drop(v);
    info!("frame_allocator_test passed!");
}