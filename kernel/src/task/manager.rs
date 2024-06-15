use super::Task;
use sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::sync::atomic::AtomicUsize;
use lazy_static::*;
use crate::loader::get_app_data_by_name;
use crate::println;
pub struct TaskManager {
    ready_tasks: VecDeque<Arc<Task>>,
    server: AtomicUsize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_tasks: VecDeque::new(),
            server: AtomicUsize::new(0),
        }
    }

    pub fn set_server(&mut self, server: AtomicUsize) {
        self.server = server;
    }

    pub fn server(&self) -> usize {
        self.server.load(core::sync::atomic::Ordering::SeqCst)
    }

    pub fn push_back(&mut self, task: Arc<Task>) {
        self.ready_tasks.push_back(task);
    }

    pub fn push_front(&mut self, task: Arc<Task>) {
        self.ready_tasks.push_front(task);
    }

    pub fn get_front(&mut self) -> Option<Arc<Task>> {
        match self.server() {
            0 => self.ready_tasks.pop_front(),
            1 => Option::from(MANAGERTASK.clone()),
            _ => None,
        }
    }

    pub fn size(&self) -> i32 {
        self.ready_tasks.len() as i32
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = UPSafeCell::new(TaskManager::new());
}

pub fn set_server(server: usize) {
    TASK_MANAGER.lock().set_server(AtomicUsize::new(server));
}

pub fn get_server() -> usize {
    TASK_MANAGER.lock().server()
}

pub fn push_back(task: Arc<Task>) {
    if get_server() == 0 {
        TASK_MANAGER.lock().push_back(task);
    } else {
        if task.pid == 1 {
            set_server(0);
        }
    }
}

pub fn push_front(task: Arc<Task>) {
    TASK_MANAGER.lock().push_front(task);
}

pub fn get_front_task() -> Option<Arc<Task>> {
    TASK_MANAGER.lock().get_front()
}

pub fn task_num() -> i32 {
    TASK_MANAGER.lock().size()
}

lazy_static! {
    pub static ref INITTASK: Arc<Task> = Arc::new(Task::new(get_app_data_by_name("initproc").expect("[task] No initproc."), 0));
    pub static ref MANAGERTASK: Arc<Task> = Arc::new(Task::new(get_app_data_by_name("manager").expect("[task] No manager."), 1));
}

pub fn init_tasks() {
    push_back(INITTASK.clone());
}