#![no_std]
#![no_main]

#[macro_use]
extern crate user;
extern crate alloc;

use alloc::sync::Arc;
use user::{alloc_pid, find_process, fork, INITPROC, insert_process, Process, remove_process, SYSCALL_EXIT, SYSCALL_FORK, SYSCALL_WAITPID, write_to_buffer, yield_};

#[no_mangle]
pub fn main() -> i32 {
    loop {
        let data: [usize; 3] = user::read_from_buffer();
        if data[0] == SYSCALL_FORK {
            let parent = find_process(data[1]).expect("[User] process in fork not found");
            let child = Arc::new(Process::new(alloc_pid()));
            child.lock().parent = Some(Arc::downgrade(&parent));
            parent.lock().children.push(child.clone());
            let child_pid = child.pid.0;
            insert_process(child.clone());
            write_to_buffer(&[child_pid]);
        } else if data[0] == SYSCALL_EXIT {
            let proc = find_process(data[1]).expect("[User] process in exit not found");
            proc.lock().exit_code = data[2] as isize;
            proc.lock().done = true;
            for child in proc.lock().children.iter() {
                child.lock().parent = Some(Arc::downgrade(&INITPROC));
                INITPROC.lock().children.push(child.clone());
            }
            proc.lock().children.clear();
            remove_process(data[1]);
        } else if data[0] == SYSCALL_WAITPID {
            let proc = find_process(data[1]).expect("[User] process in waitpid not found");
            let mut proc = proc.lock();
            let exist = proc.children.iter().any(|p| data[2] as isize == -1 || data[2] == p.pid.0);
            if !exist {
                write_to_buffer(&[0, 0, 0]);
            } else {
                let pair = proc.children.iter().enumerate().find(|(_, p)| {
                    p.lock().done && (data[2] as isize == -1 || data[2] == p.pid.0)
                });
                let mut child_pid: isize = -1;
                if let Some((idx, _)) = pair {
                    let child = proc.children.remove(idx);
                    let exit_code = child.lock().exit_code;
                    assert_eq!(Arc::strong_count(&child), 1);
                    child_pid = child.pid.0 as isize;
                    write_to_buffer(&[1, child_pid as usize, exit_code as usize]);
                } else {
                    write_to_buffer(&[1, child_pid as usize, 0]);
                }
            }
        }
        yield_();
    }
}
