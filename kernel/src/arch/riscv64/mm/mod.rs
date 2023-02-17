mod page_table;
mod page_table_entry;

use crate::mm;

pub type PageTableEntryImpl = page_table_entry::PageTableEntryImpl;
pub type PageTableNodeImpl = mm::page_table::PageTableNode<PageTableEntryImpl>;
pub type PageTableImpl = mm::page_table::PageTableTemplate<3, 9, PageTableEntryImpl>;