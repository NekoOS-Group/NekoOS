use crate::config::PAGE_SIZE;

use crate::mm;
use super::PageTableEntry;

pub struct Node<T> 
    where T: PageTableEntry + 'static
{
    page: mm::Page,
    entries: &'static mut [T],
    pub size: usize
}

impl<T> Node<T> 
    where T: PageTableEntry
{
    pub fn new(page: mm::Page) -> Self {
        Self {
            entries: unsafe{ core::slice::from_raw_parts_mut(
                page.get_bytes().as_ptr() as *mut T,
                PAGE_SIZE
            ) },
            page,
            size : 0
        }
    }

    pub fn new_alloc() -> Self {
        Self::new(mm::page_allocator::alloc().unwrap())
    }

    pub fn new_inner(parent: &mut Node<T>, index: usize) -> Self {
        let node = Self::new_alloc();
        parent.set_entry(
            index, 
            PageTableEntry::new(
                node.get_ppn(), mm::MapPermission::empty()
            )
        );
        node
    }
} 

impl<T> Node<T> 
    where T: PageTableEntry
{
    pub fn set_entry(&mut self, index: usize, entry: T) {
        *(self.entries.get_mut(index).unwrap()) = entry;
    }

    pub fn get_entry(&self, index: usize) -> T {
        self.entries[index]
    }

    pub fn get_ppn(&self) -> usize {
        self.page.ppn
    }
}