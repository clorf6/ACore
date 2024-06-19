use alloc::collections::{BinaryHeap, BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;

use lazy_static::*;
use sync::UPSafeCell;
use crate::println;
use crate::loader::get_app_data_by_name;

use super::Task;

pub struct TaskManager {
    ready_tasks: VecDeque<Arc<Task>>,
    tasks: BTreeMap<usize, Arc<Task>>,
    server: UPSafeCell<VecDeque<isize>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_tasks: VecDeque::new(),
            tasks: BTreeMap::new(),
            server: UPSafeCell::new(VecDeque::new()),
        }
    }

    pub fn set_server(&mut self, server: Vec<isize>) {
        *self.server.get() = VecDeque::from(server);
    }

    pub fn insert_task(&mut self, task: Arc<Task>) {
        self.tasks.insert(task.pid, task);
    }

    pub fn remove_task(&mut self, pid: usize) {
        self.tasks.remove(&pid);
    }

    pub fn find_task(&self, pid: usize) -> Option<Arc<Task>> {
        self.tasks.get(&pid).cloned()
    }

    pub fn server(&self) -> bool {
        !self.server.get().is_empty()
    }

    pub fn push(&mut self, task: Arc<Task>) {
        self.ready_tasks.push_back(task);
    }

    pub fn get_front(&mut self) -> Option<Arc<Task>> {
        if !self.server() {
            self.ready_tasks.pop_front()
        } else {
            let pid = self.server.get().pop_front().unwrap();
            match pid {
                -1 => self.ready_tasks.pop_front(),
                0 => Option::from(INITTASK.clone()),
                1 => Option::from(MANAGERTASK.clone()),
                _ => self.find_task(pid as usize)
            }
        }
    }

    pub fn size(&self) -> i32 {
        self.ready_tasks.len() as i32
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = UPSafeCell::new(TaskManager::new());
}

pub fn set_server(server: Vec<isize>) {
    TASK_MANAGER.get().set_server(server);
}

pub fn get_server() -> bool {
    TASK_MANAGER.get().server()
}

pub fn insert_task(task: Arc<Task>) {
    TASK_MANAGER.get().insert_task(task);
}

pub fn remove_task(pid: usize) {
    TASK_MANAGER.get().remove_task(pid);
}

pub fn find_task(pid: usize) -> Option<Arc<Task>> {
    TASK_MANAGER.get().find_task(pid)
}

pub fn push(task: Arc<Task>) {
    if !get_server() {
        TASK_MANAGER.get().push(task);
    }
}

pub fn get_front_task() -> Option<Arc<Task>> {
    TASK_MANAGER.get().get_front()
}

pub fn task_num() -> i32 {
    TASK_MANAGER.get().size()
}

lazy_static! {
    pub static ref INITTASK: Arc<Task> = Arc::new(Task::new(get_app_data_by_name("initproc").expect("[task] No initproc."), 0));
    pub static ref MANAGERTASK: Arc<Task> = Arc::new(Task::new(get_app_data_by_name("manager").expect("[task] No manager."), 1));
}

pub fn init_tasks() {
    push(INITTASK.clone());
}