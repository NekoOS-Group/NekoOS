use core::fmt;
use crate::config;
use super::page_allocator;

pub struct Page {
    pub ppn: usize,
}

impl Page {
    pub fn new(ppn: usize) -> Self {
        let ret = Page { ppn };
        debug!( "alloc {:?}", ret );
        ret
    }
}

impl Page {
    pub fn get_bytes(&self) -> &'static mut [u8] {
        unsafe{ 
            core::slice::from_raw_parts_mut(
                (self.ppn * config::PAGE_SIZE) as *mut u8, 
                config::PAGE_SIZE
            ) 
        }
    }

    pub fn set_bytes(&self, data: &[u8]) {
        self.get_bytes().copy_from_slice(data)
    }

    pub fn clear(&self) {
        self.get_bytes().fill(0);
    }
}

impl fmt::Debug for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Page<{:#x}>", self.ppn))
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        debug!("dealloc {:?}", self);
        page_allocator::dealloc(self.ppn)
    }
}