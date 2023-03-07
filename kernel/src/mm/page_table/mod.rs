mod page_table;
mod node;

pub use page_table::PageTableTemplate;
pub use node::Node as PageTableNode;

use crate::mm;

pub trait PageTableEntry: core::fmt::Debug + Copy {
    fn new(ppn: usize, permission: mm::MapPermission) -> Self;
    fn new_empty() -> Self;
    fn get_ppn(&self) -> usize;
    fn get_permission(&self) -> mm::MapPermission;
    fn is_valid(&self) -> bool;
    fn is_leaf(&self) -> bool;
}

pub trait PageTableInner<T> 
    where T: PageTableEntry
{
    fn new() -> Self;
    fn map(&mut self, vpn: usize, ppn: usize, length: usize, permission: mm::MapPermission);
    fn unmap(&mut self, vpn: usize, length: usize);
    fn query(&self, vpn: usize) -> Option<T>;
}

pub trait PageTable<T> : PageTableInner<T> + core::fmt::Debug 
    where T: PageTableEntry
{
    fn activate(&self);
}