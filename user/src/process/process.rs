use alloc::vec::Vec;
use super::{Pid, alloc_pid};

pub struct Process {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub children: Vec<Pid>,
    pub exit_code: isize,
}

impl Process {
    pub fn new() -> Self {
        let pid = Pid(0);
        Self {
            pid,
            parent: None,
            children: Vec::new(),
            exit_code: 0,
        }
    }
}