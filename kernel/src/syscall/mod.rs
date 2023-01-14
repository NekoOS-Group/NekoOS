mod fs;

pub const SYSCALL_READ: usize = 0;
pub const SYSCALL_WRITE: usize = 1;
pub const SYSCALL_EXIT: usize = 60;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    crate::println!( "syscall {} [{}, {}, {}]", syscall_id, args[0], args[1], args[2] );
    
    match syscall_id {
        SYSCALL_READ => fs::syscall_read( args[0], args[1] as *const u8, args[2] ),
        SYSCALL_WRITE => fs::syscall_write( args[0], args[1] as *const u8, args[2] ),
        SYSCALL_EXIT => {0}
        _ => panic!("unsupported syscal_id {}", syscall_id ),
    }
}