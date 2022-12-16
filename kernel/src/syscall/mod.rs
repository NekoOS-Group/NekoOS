pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    crate::println!( "syscall {} [{}, {}, {}]", syscall_id, args[0], args[1], args[2] );
    syscall_id as isize
}