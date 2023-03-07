#![allow(unused)]

use core::fmt;
use crate::{mm, println};
use mm::page_table::PageTableEntry;

bitflags! {
    pub struct Flags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

impl Flags {
    pub fn from_permission(permission: mm::MapPermission) -> Self {
        Self::from_bits(permission.bits()).unwrap() | Self::V
    }
    pub fn to_permission(&self) -> mm::MapPermission {
        mm::MapPermission::from_bits(
            self.bits & mm::MapPermission::FIELD.bits()
        ).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct PageTableEntryImpl {
    bits: usize
}

impl PageTableEntry for PageTableEntryImpl {
    fn new_empty() -> Self {
        Self { bits: 0 }
    }
    fn new(ppn: usize, permission: mm::MapPermission) -> Self {
        Self {
            bits: ppn << 10 | Flags::from_permission(permission).bits as usize,
        }
    }
    fn get_ppn(&self) -> usize {
        self.bits >> 10 & ((1usize << 44) - 1)
    }
    fn get_permission(&self) -> mm::MapPermission {
        self.get_flags().to_permission()
    }
    fn is_valid(&self) -> bool {
        (self.get_flags() & Flags::V) != Flags::empty()
    }
    fn is_leaf(&self) -> bool {
        self.is_valid() && ! self.get_permission().is_empty()
    }
}

impl PageTableEntryImpl {
    pub fn get_flags(&self) -> Flags {
        Flags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_readable(&self) -> bool {
        (self.get_flags() & Flags::R) != Flags::empty()
    }
    pub fn is_writable(&self) -> bool {
        (self.get_flags() & Flags::W) != Flags::empty()
    }
    pub fn is_executable(&self) -> bool {
        (self.get_flags() & Flags::X) != Flags::empty()
    }
    pub fn is_user_accessable(&self) -> bool {
        (self.get_flags() & Flags::U) != Flags::empty()
    }
    pub fn is_global(&self) -> bool {
        (self.get_flags() & Flags::G) != Flags::empty()
    }
    pub fn is_accessed(&self) -> bool {
        (self.get_flags() & Flags::A) != Flags::empty()
    }
    pub fn is_dirty(&self) -> bool {
        (self.get_flags() & Flags::D) != Flags::empty()
    }
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

}

impl fmt::Debug for PageTableEntryImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = if( self.is_valid() ) {"V"} else {"_"};
        let r = if( self.is_readable() ) {"R"} else {"_"};
        let w = if( self.is_writable() ) {"W"} else {"_"};
        let x = if( self.is_executable() ) {"X"} else {"_"};
        let u = if( self.is_user_accessable() ) {"U"} else {"_"};
        let g = if( self.is_global() ) {"G"} else {"_"};
        let a = if( self.is_accessed() ) {"A"} else {"_"};
        let d = if( self.is_dirty() ) {"D"} else {"_"};
        f.write_fmt(format_args!(" PTE<ppn:{:#x} flag:{}{}{}{}{}{}{}{}>", self.get_ppn(), v, r, w, x, u, g, a, d))
    }
}