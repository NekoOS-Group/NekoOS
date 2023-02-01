use crate::config::PAGE_SIZE;

use crate::mm::Page;
use crate::mm::page_allocator;
use super::page_table_entry::{ PageTableEntry, self };

pub struct Node {
    page: Page,
    entries: &'static mut [PageTableEntry],
    pub size: usize
}

impl Node {
    pub fn new(page: Page) -> Self {
        Self {
            entries: unsafe{ core::slice::from_raw_parts_mut(
                page.get_bytes().as_ptr() as *mut PageTableEntry,
                PAGE_SIZE
            ) },
            page,
            size : 0
        }
    }

    pub fn new_alloc() -> Self {
        Self::new(page_allocator::alloc().unwrap())
    }

    pub fn new_inner(parent: &mut Node, index: usize) -> Self {
        let node = Self::new_alloc();
        parent.set_entry(
            index, 
            PageTableEntry::new(
                node.get_ppn(), page_table_entry::Flags::V
            )
        );
        node
    }
} 

impl Node {
    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry) {
        *(self.entries.get_mut(index).unwrap()) = entry;
    }

    pub fn get_entry(&self, index: usize) -> PageTableEntry {
        self.entries[index]
    }

    pub fn get_ppn(&self) -> usize {
        self.page.ppn
    }
}