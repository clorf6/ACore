use alloc::sync::Arc;
use core::cell::RefMut;

use sync::UPSafeCell;

use crate::config::TRAP_CONTEXT;
use crate::mm::{KERNEL_SPACE, MemorySet, PhysPageNum, VirtAddr};
use crate::trap::{trap_handler, trap_return, TrapContext};

use super::KernelStack;

#[derive(Eq, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn new(ra: usize, sp: usize) -> Self {
        Self {
            ra,
            sp,
            s: [0; 12],
        }
    }
}

pub struct Task {
    pub pid: usize,
    pub kernel_stack: KernelStack,
    pub inner: UPSafeCell<TaskInner>,
}

pub struct TaskInner {
    pub trap_ctx: PhysPageNum,
    pub task_ctx: TaskContext,
    pub task_status: TaskStatus,
    pub memory: MemorySet,
}

impl TaskInner {
    pub fn trap_ctx(&self) -> &'static mut TrapContext {
        self.trap_ctx.get_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memory.token()
    }
}

impl Drop for TaskInner {
    fn drop(&mut self) {
        self.task_status = TaskStatus::Zombie;
        self.memory.clean();
    }
}

impl Task {
    pub fn lock(&self) -> RefMut<'_, TaskInner> {
        self.inner.get()
    }

    pub fn new(elf_data: &[u8], pid: usize) -> Self {
        let (mut memory_set, user_sp, user_sepc) = MemorySet::from_elf(elf_data);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let kernel_stack = KernelStack::new(pid);
        let kernel_stack_top = kernel_stack.top();
        let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
        *trap_ctx = TrapContext::app_init_context(
            user_sepc,
            user_sp,
            KERNEL_SPACE.get().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        memory_set.map_buffer(pid);
        Self {
            pid,
            kernel_stack,
            inner: UPSafeCell::new(TaskInner {
                    trap_ctx: trap_ctx_ppn,
                    task_ctx: TaskContext::new(trap_return as usize, kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory: memory_set,
                }),
        }
    }

    pub fn fork(self: &Arc<Self>, pid: usize) -> Arc<Self> {
        let parent_inner = self.inner.get();
        let mut memory_set = MemorySet::from_user(&parent_inner.memory);
        drop(parent_inner);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let kernel_stack = KernelStack::new(pid);
        let kernel_stack_top = kernel_stack.top();
        let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
        trap_ctx.kernel_sp = kernel_stack_top;
        memory_set.map_buffer(pid);
        Arc::new( Self {
            pid,
            kernel_stack,
            inner: UPSafeCell::new(TaskInner {
                    trap_ctx: trap_ctx_ppn,
                    task_ctx: TaskContext::new(trap_return as usize, kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory: memory_set,
                }),
        })
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (mut memory_set, user_sp, user_sepc) = MemorySet::from_elf(elf_data);
        memory_set.map_buffer(self.pid);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let mut inner = self.lock();
        inner.memory = memory_set;
        inner.trap_ctx = trap_ctx_ppn;
        drop(inner);
        let trap_ctx = trap_ctx_ppn.get_mut();
        *trap_ctx = TrapContext::app_init_context(
            user_sepc,
            user_sp,
            KERNEL_SPACE.get().token(),
            self.kernel_stack.top(),
            trap_handler as usize,
        );
    }
}

