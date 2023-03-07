mod buddy_allocator;
mod stack_allocator;

pub trait Allocator {
    fn new() -> Self;
    fn add(&mut self, l: usize, r:usize);
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, ppn: usize);
}

pub use buddy_allocator::BuddyAllocator;
pub use stack_allocator::StackAllocator;