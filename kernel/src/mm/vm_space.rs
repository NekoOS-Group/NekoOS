use alloc::vec::Vec;

use crate::mm;
use crate::mm::page_table::PageTable;

pub struct VmSpaceImpl<T> 
    where T: PageTable<mm::PageTableEntry>
{
    segments: Vec<mm::Segment>,
    page_table: T,
}

impl<T> VmSpaceImpl<T> 
    where T: PageTable<mm::PageTableEntry>
{
    pub fn new() -> Self {
        Self { 
            segments: Vec::new(), 
            page_table: T::new()
        }
    }
}

impl<T> VmSpaceImpl<T> 
    where T: PageTable<mm::PageTableEntry>
{
    pub fn push(&mut self, mut segment: mm::Segment, data: Option<&[u8]>) {
        segment.map_all(&mut self.page_table);
        if let Some(data) = data {
            segment.copy_data(&mut self.page_table, data)
        }
        self.segments.push(segment);
    }
    pub fn get_page_table(&mut self) -> &mut T {
        &mut self.page_table
    }
}