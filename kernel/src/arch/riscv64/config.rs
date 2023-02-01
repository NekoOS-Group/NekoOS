#![allow(unused)]

pub const ARCH : &str = "riscv64";
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xFFFF_FFFF_4000_0000;
pub const KERNEL_OFFSET:          usize = 0xFFFF_FFFF_C000_0000;
pub const MEMORY_START_ADDRESS:   usize = 0x80000000;
pub const EARLY_MEMORY_END:       usize = 0x88000000;