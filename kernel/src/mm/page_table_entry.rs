#![allow(unused)]

use core::fmt;

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

#[derive(Copy, Clone)]
pub struct PageTableEntry {
    bits: usize
}

impl PageTableEntry {
    fn new_empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    fn new(ppn: usize, flags: Flags) -> Self {
        PageTableEntry {
            bits: ppn << 10 | flags.bits as usize,
        }
    }
}

impl PageTableEntry {
    fn get_ppn(&self) -> usize {
        self.bits >> 10 & ((1usize << 44) - 1)
    }
    fn get_flags(&self) -> Flags {
        Flags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.get_flags() & Flags::V) != Flags::empty()
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
    pub fn is_accessed(&self) -> bool {
        (self.get_flags() & Flags::A) != Flags::empty()
    }
    pub fn is_dirty(&self) -> bool {
        (self.get_flags() & Flags::D) != Flags::empty()
    }
}

impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(" PTE[page: {} flag:{:#b}]", self.get_ppn(), self.get_flags()))
    }
}