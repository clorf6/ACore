use crate::task::exit_and_yield;

pub fn sys_exit(exit_code: isize) -> ! {
    exit_and_yield(exit_code);
    panic!("Unreachable in sys_exit!");
}
