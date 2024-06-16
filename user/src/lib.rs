#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
pub mod process;
mod exception;
mod syscall;

use allocator::BuddyAllocator;
pub use syscall::*;
pub use process::*;
//use buddy_system_allocator::LockedHeap;

extern crate alloc;
const USER_HEAP_SIZE: usize = 16384;
const ALLOC_MINIMUM: usize = 256;

const fn get_alloc_num(total: usize, minimum: usize) -> usize {
    2 * total / minimum
}

const ALLOC_NUM: usize = get_alloc_num(USER_HEAP_SIZE, ALLOC_MINIMUM);

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
//static HEAP: LockedHeap = LockedHeap::empty();
static HEAP: BuddyAllocator<ALLOC_MINIMUM, ALLOC_NUM> = BuddyAllocator::new();

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    unsafe {
        let begin = HEAP_SPACE.as_ptr() as usize;
        HEAP.init(begin);
        //HEAP.lock().init(begin, USER_HEAP_SIZE);
    }
    init_processes();
    exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str) -> isize {
    sys_exec(path)
}
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            exit_pid => return exit_pid,
        }
    }
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            exit_pid => return exit_pid,
        }
    }
}
pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}
