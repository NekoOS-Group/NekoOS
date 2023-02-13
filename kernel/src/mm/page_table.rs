use crate::mm;

pub trait PageTable {
    fn new() -> Self;
    fn map(&mut self, vpn: usize, ppn: usize, length: usize, flags: PageFlagImpl);
    fn unmap(&mut self, vpn: usize, length: usize);
    fn activate(&self);
    fn query_ppn(&self, vpn: usize) -> Option<usize>;
    fn query_permission(&self, vpn: usize) -> mm::MapPermission;
}

pub trait PageFlag {
    fn from_permission(permission: mm::MapPermission) -> Self;
}

pub type PageTableImpl = crate::arch::mm::PageTable<3, 9>;
pub type PageFlagImpl = crate::arch::mm::PageFlag;