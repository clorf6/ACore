use crate::loader::get_app_data_by_name;
use crate::mm::buffer::{read_from_buffer, write_to_buffer};
use crate::mm::page_table::{translated_refmut, translated_string};
use crate::syscall::{SYSCALL_EXIT, SYSCALL_FORK, SYSCALL_WAITPID};
use crate::task::*;
use crate::println;
use crate::console::shutdown;
use crate::time::get_time;

pub fn sys_exit(exit_code: isize) -> ! {
    let pid = get_cur_task().pid;
    if pid == 0 {
        println!(
            "[kernel] INITPROC exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            shutdown(true)
        } else {
            shutdown(false)
        }
    }
    write_to_buffer(&[SYSCALL_EXIT, pid, exit_code as usize], 1);
    set_server(1);
    exit_and_yield();
    unreachable!("Unreachable in sys_exit");
}

pub fn sys_getpid() -> isize {
    get_cur_task().pid as isize
}

pub fn sys_yield() -> isize {
    suspend_and_yield();
    0
}

pub fn sys_fork() -> isize {
    let task = get_cur_task();
    let pid = task.pid;
    push_front(task);
    write_to_buffer(&[SYSCALL_FORK, pid], 1);
    set_server(1);
    suspend_and_yield();
    let [new_pid] = read_from_buffer(1); // child pid
    let new_task = get_cur_task().fork(new_pid);
    let trap_ctx = new_task.trap_ctx();
    trap_ctx.x[10] = 0;
    push_back(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = get_cur_task().user_token();
    let path = translated_string(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        get_cur_task().exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut isize) -> isize {
    let task = get_cur_task();
    let cur_pid = task.pid;
    let token = task.user_token();
    push_front(task);
    write_to_buffer(&[SYSCALL_WAITPID, cur_pid, pid as usize], 1);
    set_server(1);
    suspend_and_yield();
    let [exist, child_pid, exit_code]: [usize; 3] = read_from_buffer(1);
    let child_pid: isize = child_pid as isize;
    let exit_code: isize = exit_code as isize;
    if exist == 0 {
        -1
    } else {
        if child_pid == -1 {
            -2
        } else {
            *translated_refmut(token, exit_code_ptr) = exit_code;
            child_pid
        }
    }
}

pub fn sys_get_time() -> isize {
    get_time() as isize
}