use super::Task;
use spin::Mutex;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
use crate::loader::{get_app_data, get_app_data_by_name, get_num_app};
use crate::println;
pub struct TaskManager {
    ready_tasks: VecDeque<Arc<Task>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_tasks: VecDeque::new(),
        }
    }

    pub fn push(&mut self, task: Arc<Task>) {
        self.ready_tasks.push_back(task);
    }

    pub fn front(&mut self) -> Option<Arc<Task>> {
        self.ready_tasks.pop_front()
    }

    pub fn size(&self) -> i32 {
        self.ready_tasks.len() as i32
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn push_task(task: Arc<Task>) {
    TASK_MANAGER.lock().push(task);
}

pub fn front_task() -> Option<Arc<Task>> {
    TASK_MANAGER.lock().front()
}

pub fn task_num() -> i32 {
    TASK_MANAGER.lock().size()
}

pub fn init_tasks() {
    let init_task = Task::new(get_app_data_by_name("initproc").expect("[task] No initproc."));
    push_task(Arc::new(init_task));
}