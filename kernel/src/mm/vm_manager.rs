use alloc::vec::Vec;

use crate::mm;
use crate::mm::page_table::PageTable;

pub struct VmManagerImpl {
    segments: Vec<mm::Segment>,
    page_table: mm::PageTable,
}

impl VmManagerImpl {
    pub fn new() -> Self {
        Self { 
            segments: Vec::new(), 
            page_table: mm::PageTable::new()
        }
    }
}

impl VmManagerImpl {
    pub fn query_ppn(&self, vpn: usize) -> Option<usize> {
        self.page_table.query_ppn(vpn)
    }
    pub fn push(&mut self, mut segment: mm::Segment, data: Option<&[u8]>) {
        segment.map_all(&mut self.page_table);
        if let Some(data) = data {
            segment.copy_data(&mut self.page_table, data)
        }
        self.segments.push(segment);
    }
    pub fn get_page_table(&mut self) -> &mut mm::PageTable {
        &mut self.page_table
    }
}