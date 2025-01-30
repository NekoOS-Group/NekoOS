use crate::mm;
use mm::KERNEL_SPACE;
use mm::page_table::PageTable;

pub fn init(memory: &fdt::standard_nodes::Memory) {
    use crate::config::{ skernel, ekernel, stext, etext, srodata, erodata, sdata, edata, sbss, ebss, PHYSICAL_MEMORY_OFFSET, PAGE_SIZE };
    unsafe {
        KERNEL_SPACE.lock().replace(mm::VmSpace::new());
        if let Some(inner) = KERNEL_SPACE.lock().as_mut() {
            inner.push(
                mm::Segment::new(
                    ".text",
                    stext as usize / PAGE_SIZE,
                    etext as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::X
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".rodata",
                    srodata as usize / PAGE_SIZE,
                    erodata as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".data",
                    sdata as usize / PAGE_SIZE,
                    edata as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R | mm::MapPermission::W
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".bss",
                    sbss as usize / PAGE_SIZE,
                    ebss as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R | mm::MapPermission::W
                ), 
                None
            );
            for region in memory.regions() {
                let mut l = region.starting_address as usize + PHYSICAL_MEMORY_OFFSET;
                let r = region.starting_address as usize + region.size.unwrap() + PHYSICAL_MEMORY_OFFSET;
                if l <= skernel as usize && r >= ekernel as usize {
                    l = ekernel as usize;
                }
                inner.push(
                    mm::Segment::new(
                        "free space",
                        l / PAGE_SIZE,
                        r / PAGE_SIZE,
                        mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                        mm::MapPermission::R | mm::MapPermission::W
                    ), 
                    None
                );
            }
        }
    }
    on();
    crate::println!("[Neko] kernel space inited.");
}

pub fn on() {
    unsafe {
        if let Some(inner) = KERNEL_SPACE.lock().as_mut() {
            inner.get_page_table().activate();
        }
    }
}   

#[cfg(debug_assertions)]
pub fn test() {
    use mm::page_table::PageTableInner;
    use mm::page_table::PageTableEntry;
    use crate::config::{ stext, etext, srodata, erodata, sdata, edata, PAGE_SIZE };
    unsafe {
        if let Some(kernel_space) = KERNEL_SPACE.lock().as_mut() {
            let mid_text = stext as usize / 2 + etext as usize / 2;
            let mid_rodata = srodata as usize / 2+ erodata as usize/ 2;
            let mid_data = sdata as usize / 2 + edata as usize / 2;
            assert!(!kernel_space
                .get_page_table()
                .query(mid_text / PAGE_SIZE).unwrap()
                .get_permission().is_readable());
            assert!(!kernel_space
                .get_page_table()
                .query(mid_rodata / PAGE_SIZE).unwrap()
                .get_permission()
                .is_writable());
            assert!(!kernel_space
                .get_page_table()
                .query(mid_data / PAGE_SIZE).unwrap()
                .get_permission()
                .is_executable());
            info!("kernel space remap test passed!");
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn test() {}