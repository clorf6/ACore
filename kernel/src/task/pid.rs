use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::*;

pub struct Pid(pub usize);

impl Drop for Pid {
    fn drop(&mut self) {
        dealloc_pid(self);
    }
}

pub struct PidAllocator {
    cur: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            cur: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Pid {
        if let Some(pid) = self.recycled.pop() {
            Pid(pid)
        } else {
            self.cur += 1;
            Pid(self.cur - 1)
        }
    }

    pub fn dealloc(&mut self, pid: &Pid) {
        self.recycled.push(pid.0);
    }
}

lazy_static! {
    pub static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> = unsafe { UPSafeCell::new(PidAllocator::new()) };
}

pub fn alloc_pid() -> Pid {
    PID_ALLOCATOR.get().alloc()
}

pub fn dealloc_pid(pid: &Pid) {
    PID_ALLOCATOR.get().dealloc(pid);
}