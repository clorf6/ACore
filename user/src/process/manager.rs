use super::Process;
use super::Pid;
use spin::Mutex;
use alloc::collections::BTreeMap;
use lazy_static::*;

pub struct ProcessManager {
    processes: BTreeMap<usize, Process>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, process: Process) {
        self.processes.insert(process.pid.0, process);
    }

    pub fn remove(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    pub fn size(&self) -> i32 { self.processes.len() as i32 }
}

lazy_static!{
    pub static ref PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
}

pub fn insert_process(process: Process) {
    PROCESS_MANAGER.lock().insert(process);
}

pub fn remove_process(pid: usize) {
    PROCESS_MANAGER.lock().remove(pid);
}

pub fn process_num() -> i32 {
    PROCESS_MANAGER.lock().size()
}

pub fn init_processes() {
    let init_process = Process::new();
    //insert_process(Arc::new(init_process));
}