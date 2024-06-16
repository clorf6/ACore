#![no_std]
#![no_main]

extern crate alloc;
extern crate user;

use alloc::sync::Arc;

use user::{alloc_pid, find_process, INITPROC, insert_process, Process, remove_process, SYSCALL_EXIT, SYSCALL_FORK, SYSCALL_WAITPID, write_to_buffer, yield_};

#[no_mangle]
pub fn main() -> i32 {
    loop {
        let data: [usize; 3] = user::read_from_buffer();
        if data[0] == SYSCALL_FORK {
            let parent = find_process(data[1]).expect("[User] process in fork not found");
            let child = Arc::new(Process::new(alloc_pid()));
            let child_pid = child.pid.0;
            let mut child_lock = child.lock();
            child_lock.parent = Some(Arc::downgrade(&parent));
            drop(child_lock);
            let child_2 = child.clone();
            insert_process(child);
            let mut parent_lock = parent.lock();
            parent_lock.children.push(child_2);
            drop(parent_lock);
            write_to_buffer(&[child_pid]);
        } else if data[0] == SYSCALL_EXIT {
            let proc = find_process(data[1]).expect("[User] process in exit not found");
            let mut proc = proc.lock();
            proc.exit_code = data[2] as isize;
            proc.done = true;
            for child in proc.children.iter() {
                child.lock().parent = Some(Arc::downgrade(&INITPROC));
                INITPROC.lock().children.push(child.clone());
            }
            proc.children.clear();
            drop(proc);
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
                    drop(child);
                } else {
                    write_to_buffer(&[1, child_pid as usize, 0]);
                }
            }
            drop(proc);
        }
        yield_();
    }
}
