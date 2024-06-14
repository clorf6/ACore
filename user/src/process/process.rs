use alloc::sync::{Weak, Arc};
use alloc::vec::Vec;
use spin::Mutex;
use super::{Pid, alloc_pid};

pub struct Process {
    pub pid: Pid,

    inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    pub done: bool,
    pub parent: Option<Weak<Process>>,
    pub children: Vec<Arc<Process>>,
    pub exit_code: isize,
}

impl Process {
    pub fn lock(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            inner: unsafe {
                Mutex::new(ProcessInner {
                    done: false,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                })
            },
        }
    }
}