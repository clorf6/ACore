use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE};
use crate::mm::{MapPermission, VirtPageNum, KERNEL_SPACE};

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE); // not use guard page
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    pub fn new(pid: usize) -> Self {
        let (bottom, top) = kernel_stack_position(pid);
        KERNEL_SPACE.get().map(
            bottom.into(),
            top.into(),
            MapPermission::R | MapPermission::W,
        );
        Self { pid }
    }

    pub fn top(&self) -> usize {
        let (_, top) = kernel_stack_position(self.pid);
        top
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (bottom, _) = kernel_stack_position(self.pid);
        let bottom: VirtPageNum = bottom.into();
        KERNEL_SPACE.get().unmap(bottom);
    }
}