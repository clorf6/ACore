use alloc::sync::Arc;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::println;
use crate::task::manager::get_server;

use super::{__switch, get_front_task, push_back, Task, TaskContext, TaskStatus};

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

lazy_static! {
    pub static ref PROCESSOR: Mutex<Processor> = Mutex::new(Processor::new());
}

pub fn get_cur_task() -> Arc<Task> {
    PROCESSOR.lock().cur_task().expect("[kernel] No running task currently.")
}

pub fn take_cur_task() -> Arc<Task> {
    PROCESSOR.lock().take_cur_task().expect("[kernel] No running task currently.")
}

pub fn get_idle_task_ctx() -> *mut TaskContext {
    PROCESSOR.lock().idle_task_ctx_ptr()
}

pub fn run_tasks() {
    loop {
        let task = get_front_task();
        if let Some(task) = task {
            task.inner.lock().task_status = TaskStatus::Running;
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

pub fn schedule(status: TaskStatus, add: bool) {
    let idle_task_ctx_ptr = get_idle_task_ctx();
    let task = take_cur_task();
    let mut task_ctx_ptr = task.task_ctx_ptr() as *mut TaskContext;
    if status == TaskStatus::Zombie {
        task.inner.lock().task_status = status;
        task.inner.lock().memory.clean();
        task_ctx_ptr = &mut TaskContext::empty() as *mut _;
    }
    if add { push_back(task); }
    unsafe { __switch(task_ctx_ptr, idle_task_ctx_ptr) };
}
