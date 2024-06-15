use super::Process;
use super::Pid;
use sync::UPSafeCell;
use alloc::sync::{Weak, Arc};
use alloc::collections::BTreeMap;
use lazy_static::*;

pub struct ProcessManager {
    processes: BTreeMap<usize, Arc<Process>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, process: Arc<Process>) {
        self.processes.insert(process.pid.0, process);
    }

    pub fn remove(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    pub fn find(&self, pid: usize) -> Option<Arc<Process>> {
        self.processes.get(&pid).cloned()
    }

    pub fn size(&self) -> i32 { self.processes.len() as i32 }
}

lazy_static!{
    pub static ref PROCESS_MANAGER: UPSafeCell<ProcessManager> = UPSafeCell::new(ProcessManager::new());
}

pub fn insert_process(process: Arc<Process>) {
    PROCESS_MANAGER.lock().insert(process);
}

pub fn remove_process(pid: usize){
    PROCESS_MANAGER.lock().remove(pid);
}

pub fn find_process(pid: usize) -> Option<Arc<Process>> {
    PROCESS_MANAGER.lock().find(pid)
}

pub fn process_num() -> i32 {
    PROCESS_MANAGER.lock().size()
}

lazy_static! {
    pub static ref INITPROC: Arc<Process> = Arc::new(Process::new(Pid(0)));
    pub static ref MANAGERPROC: Arc<Process> = Arc::new(Process::new(Pid(1)));
}

pub fn init_processes() {
    MANAGERPROC.lock().parent = Some(Arc::downgrade(&INITPROC));
    INITPROC.lock().children.push(MANAGERPROC.clone());
    insert_process(INITPROC.clone());
    insert_process(MANAGERPROC.clone());
}