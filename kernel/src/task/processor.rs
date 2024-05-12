use spin::Mutex;
use super::{__switch, front_task, push_task, Task, TaskContext, TaskStatus};
use alloc::sync::Arc;
use lazy_static::lazy_static;
use crate::println;
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

    fn idle_task_ctx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_ctx as *mut _
    }

    pub fn cur_task(&mut self) -> Option<Arc<Task>> {
        self.cur.clone()
    }

    pub fn take_cur_task(&mut self) -> Option<Arc<Task>> {
        self.cur.take()
    }
}

pub fn get_cur_task() -> Arc<Task> {
    PROCESSOR.lock().cur_task().expect("[kernel] No running task currently.")
}

pub fn get_idle_task_ctx() -> *mut TaskContext {
    PROCESSOR.lock().idle_task_ctx_ptr()
}

lazy_static! {
    pub static ref PROCESSOR: Mutex<Processor> = Mutex::new(Processor::new());
}

pub fn run_tasks() {
    loop {
        let task = front_task();
        if let Some(task) = task {
            let idle_task_ctx_ptr = get_idle_task_ctx();
            let task_ctx_ptr = task.task_ctx_ptr();
            PROCESSOR.lock().cur = Some(task); 
            unsafe {
                __switch(idle_task_ctx_ptr, task_ctx_ptr);
            }
        } else {
            panic!("[kernel] No running task!");
        }
    }
}

pub fn schedule(status: TaskStatus, add: bool, exit_code: isize) {
    let idle_task_ctx_ptr = get_idle_task_ctx();
    let task = get_cur_task();
    task.inner.lock().task_status = status;
    task.inner.lock().exit_code = exit_code;
    let task_ctx_ptr = task.task_ctx_ptr() as *mut TaskContext;
    if add { push_task(task); }
    unsafe { __switch(task_ctx_ptr, idle_task_ctx_ptr) };
}
