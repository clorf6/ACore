use crate::loader::get_app_data_by_name;
use crate::mm::buffer::{read_from_buffer, write_to_buffer};
use crate::mm::page_table::{translated_refmut, translated_string};
use crate::syscall::{SYSCALL_FORK, SYSCALL_WAITPID};
use crate::task::*;
use crate::time::get_time;
use alloc::vec;

pub fn sys_exit(exit_code: isize) -> ! {
    set_server(vec![1, -1]);
    exit_and_yield(exit_code);
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
    write_to_buffer(&[SYSCALL_FORK, pid], 1);
    //println!("fork");
    set_server(vec![1, pid as isize]);
    suspend_and_yield();
    let [new_pid] = read_from_buffer(1); // child pid
    let new_task = get_cur_task().fork(new_pid);
    let trap_ctx = new_task.lock().trap_ctx();
    trap_ctx.x[10] = 0;
    push(new_task.clone());
    insert_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = user_token();
    let path = translated_string(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        get_cur_task().exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = get_cur_task();
    let cur_pid = task.pid;
    let token = user_token();
    write_to_buffer(&[SYSCALL_WAITPID, cur_pid, pid as usize], 1);
    set_server(vec![1, cur_pid as isize]);
    suspend_and_yield();
    let [exist, child_pid, exit_code]: [usize; 3] = read_from_buffer(1);
    let child_pid: isize = child_pid as isize;
    let exit_code: i32 = exit_code as i32;
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
    (get_time() / 10000) as isize
}
