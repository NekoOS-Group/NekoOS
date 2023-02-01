use crate::mm;

pub trait PageTable {
    fn new() -> Self;
    fn map(&mut self, vpn: usize, ppn: usize, flags: PageFlagImpl);
    fn unmap(&mut self, vpn: usize);
    fn activate(&self);
    fn query_ppn(&self, vpn: usize) -> Option<usize>;
}

pub trait PageFlag {
    fn from_permission(permission: mm::MapPermission) -> Self;
}

pub type PageTableImpl = crate::arch::mm::PageTable;
pub type PageFlagImpl = crate::arch::mm::PageFlag;