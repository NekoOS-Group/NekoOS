#![allow(unused)]

use crate::arch::trap::syscall::*;
use crate::fs;

pub enum SyscallErorr {
    
}

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    crate::println!( "syscall {} [{}, {}, {}]", syscall_id, args[0], args[1], args[2] );
    
    match syscall_id {
        SYS_READ => fs::sys_read( args[0], args[1] as *const u8, args[2] as usize ),
        SYS_WRITE => fs::sys_write( args[0], args[1] as *const u8, args[2] as usize ),
        SYS_EXIT => {0}
        _ => panic!("unsupported syscal_id {}", syscall_id ),
    }
}
