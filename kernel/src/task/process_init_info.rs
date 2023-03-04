use alloc::{vec::Vec, string::String};
use alloc::collections::btree_map::BTreeMap;

pub struct ProcessInitInfo {
    args: Vec<String>,
    envs: Vec<String>,
    auxv: BTreeMap<u8, usize>
}

pub enum AuxIndex {
    AT_NULL         = 0,
    AT_IGNORE       = 1,
    AT_EXECFD       = 2,
    AT_PHDR         = 3,
    AT_PHENT        = 4,
    AT_PHNUM        = 5,
    AT_PAGESZ       = 6,
    AT_BASE         = 7,
    AT_FLAGS        = 8,
    AT_ENTRY        = 9,
    AT_NOTELF       = 10,
    AT_UID          = 11,
    AT_EUID         = 12,
    AT_GID          = 13,
    AT_EGID         = 14,
    AT_CLKTCK       = 15,
    AT_PLATFORM     = 16,
    AT_HWCAP        = 17,
    AT_FPUCW        = 18,
    AT_DCACHEBSIZE  = 19,
    AT_ICACHEBSIZE  = 20,
    AT_UCACHEBSIZE  = 21,
}