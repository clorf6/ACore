use crate::print;
use crate::task::get_cur_task;
use crate::mm::translated_byte_buffer;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let token = get_cur_task().user_token();
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