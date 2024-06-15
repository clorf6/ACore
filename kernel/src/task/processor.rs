use alloc::sync::Arc;

use lazy_static::lazy_static;
use sync::UPSafeCell;
use crate::trap::TrapContext;
use crate::console::shutdown;
use crate::syscall::SYSCALL_EXIT;
use crate::mm::buffer::write_to_buffer;
use crate::println;
use crate::task::manager::get_server;

use super::{__switch, get_front_task, KernelStack, push_back, set_server, Task, task_num, TaskContext, TaskStatus};

pub struct Processor {
    cur: Option<Arc<Task>>,
    idle_task_ctx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            cur: None,
            idle_task_ctx: TaskContext::empty(),
        }
    }

    pub fn idle_task_ctx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_ctx as *mut _
    }

    pub fn cur_task(&mut self) -> Option<Arc<Task>> {
        self.cur.as_ref().map(Arc::clone)
    }

    pub fn take_cur_task(&mut self) -> Option<Arc<Task>> {
        self.cur.take()
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = UPSafeCell::new(Processor::new());
}

pub fn get_cur_task() -> Arc<Task> {
    PROCESSOR.lock().cur_task().expect("[kernel] No running task currently.")
}

pub fn take_cur_task() -> Arc<Task> {
    PROCESSOR.lock().take_cur_task().expect("[kernel] No running task currently.")
}

pub fn user_token() -> usize {
    let task = get_cur_task();
    let token = task.lock().user_token();
    token
}

pub fn trap_ctx() -> &'static mut TrapContext {
    get_cur_task().lock().trap_ctx()
}

pub fn run_tasks() {
    loop {
        let task = get_front_task();
        if let Some(task) = task {
            let mut inner = task.inner.lock();
            inner.task_status = TaskStatus::Running;
            let task_ctx_ptr = &inner.task_ctx as *const TaskContext;
            drop(inner);
            let mut processor = PROCESSOR.lock();
            let idle_task_ctx_ptr = processor.idle_task_ctx_ptr();
            //println!("run task {} server {} num {}", task.pid, get_server(), task_num());
            processor.cur = Some(task);
            drop(processor);
            unsafe {
                __switch(idle_task_ctx_ptr, task_ctx_ptr);
            }
        } else {
            panic!("[kernel] No running task!");
        }
    }
}

pub fn schedule(add: bool, exit_code: isize) {
    let task = take_cur_task();
    let mut inner = task.inner.lock();
    //println!("schedule task {} server {} num {}", task.pid, get_server(), task_num());
    if add { 
        inner.task_status = TaskStatus::Ready;
        let mut task_ctx_ptr = &mut inner.task_ctx as *mut TaskContext;
        drop(inner);
        push_back(task);
        let mut processor = PROCESSOR.lock();
        let idle_task_ctx_ptr = processor.idle_task_ctx_ptr();
        drop(processor);
        unsafe { __switch(task_ctx_ptr, idle_task_ctx_ptr) };
    }
    else {
        let pid = task.pid;
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
        drop(inner);
        drop(task);
        let mut _unused = TaskContext::empty();
        let mut processor = PROCESSOR.lock();
        let idle_task_ctx_ptr = processor.idle_task_ctx_ptr();
        drop(processor);
        unsafe { __switch(&mut _unused as *mut _, idle_task_ctx_ptr) };
    }

}
