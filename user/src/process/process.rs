use alloc::sync::{Weak, Arc};
use alloc::vec::Vec;
use core::cell::RefMut;
use sync::UPSafeCell;
use super::Pid;

pub struct Process {
    pub pid: Pid,
    inner: UPSafeCell<ProcessInner>,
}

pub struct ProcessInner {
    pub done: bool,
    pub parent: Option<Weak<Process>>,
    pub children: Vec<Arc<Process>>,
    pub exit_code: isize,
}

impl Process {
    pub fn lock(&self) -> RefMut<'_, ProcessInner> {
        self.inner.get()
    }
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            inner: UPSafeCell::new(ProcessInner {
                    done: false,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                }),
        }
    }
}