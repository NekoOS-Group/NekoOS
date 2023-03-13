use alloc::{vec::Vec, string::String};
use alloc::collections::btree_map::BTreeMap;

pub struct ProcessInitInfo {
    args: Vec<String>,
    envs: Vec<String>,
    auxv: BTreeMap<u8, usize>
}


pub enum AuxIndex {
    NULL         = 0,
    IGNORE       = 1,
    EXECFD       = 2,
    PHDR         = 3,
    PHENT        = 4,
    PHNUM        = 5,
    PAGESZ       = 6,
    BASE         = 7,
    FLAGS        = 8,
    ENTRY        = 9,
    NOTELF       = 10,
    UID          = 11,
    EUID         = 12,
    GID          = 13,
    EGID         = 14,
    CLKTCK       = 15,
    PLATFORM     = 16,
    HWCAP        = 17,
    FPUCW        = 18,
    DCACHEBSIZE  = 19,
    ICACHEBSIZE  = 20,
    UCACHEBSIZE  = 21,
}