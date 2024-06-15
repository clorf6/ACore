use crate::console::getchar;
use crate::mm::translated_byte_buffer;
use crate::print;
use crate::task::{user_token, suspend_and_yield};

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let token = user_token();
            let buffers = translated_byte_buffer(token, buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Only support len = 1 in sys_read!");
            let mut ch: u8;
            loop {
                ch = getchar();
                if ch == 0 {
                    suspend_and_yield();
                    continue;
                } else {
                    break;
                }
            }
            let token = user_token();
            let mut buffers = translated_byte_buffer(token, buf, len);
            unsafe { buffers[0].as_mut_ptr().write_volatile(ch) }
            1
        }
        _ => {
            panic!("Unsupported fd in sys_read!");
        }
    }
}