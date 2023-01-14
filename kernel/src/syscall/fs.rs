const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 0;


pub fn syscall_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            -1
        }
        _ => { panic!("Unsupported fd for sys_read"); }   
    }
}

pub fn syscall_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe{ core::slice::from_raw_parts(buf, len) };
            let data = core::str::from_utf8(slice).unwrap();
            crate::print!( "{}", data );
            len as isize
        }
        _ => { panic!("Unsupported fd for sys_write"); }
    }
}