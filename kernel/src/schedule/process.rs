#![allow(unused)]
use alloc::vec::Vec;
use alloc::sync::Weak;

use crate::mm;

pub enum ProcessState {
    Active,
    Blocked,
    Stopped{
        exit_code : isize
    }
}

pub struct ProcessStats {
    utime: u64,
    stime: u64,
    start_time: u64,
    trap_count: usize,
}

pub struct ProcessInfos {

}

pub struct Process {
    pid: usize,
    state: ProcessState,
    stats: ProcessStats,
    infos: ProcessInfos,

    parent: Option<Weak<Process>>,
    children: Vec<Process>,

    vm_space: mm::VmSpace,
    kernel_stack: (),

}