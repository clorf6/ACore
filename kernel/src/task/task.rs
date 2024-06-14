use crate::mm::{PhysPageNum, VirtAddr, MemorySet, KERNEL_SPACE};
use spin::{Mutex, MutexGuard};
use alloc::sync::{Arc, Weak};
use crate::config::{TRAP_CONTEXT, PAGE_SIZE};
use crate::trap::{TrapContext, trap_return, trap_handler};
use super::{KernelStack};
use crate::println;

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
    pub inner: Mutex<TaskInner>,
}

pub struct TaskInner {
    pub trap_ctx: PhysPageNum,
    pub task_ctx: TaskContext,
    pub task_status: TaskStatus,
    pub memory: MemorySet,
}

impl Task {
    pub fn lock(&self) -> MutexGuard<TaskInner> {
        self.inner.lock()
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
            inner: Mutex::new(TaskInner {
                    trap_ctx: trap_ctx_ppn,
                    task_ctx: TaskContext::new(trap_return as usize, kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory: memory_set,
                }),
        }
    }

    pub fn fork(self: &Arc<Self>, pid: usize) -> Arc<Self> {
        let mut parent_inner = self.inner.lock();
        let memory_set = MemorySet::from_user(&parent_inner.memory);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let kernel_stack = KernelStack::new(pid);
        let kernel_stack_top = kernel_stack.top();
        let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
        trap_ctx.kernel_sp = kernel_stack_top;
        Arc::new( Self {
            pid,
            kernel_stack,
            inner: unsafe {
                Mutex::new(TaskInner {
                    trap_ctx: trap_ctx_ppn,
                    task_ctx: TaskContext::new(trap_return as usize, kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory: memory_set,
                })
            },
        })
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (memory_set, user_sp, user_sepc) = MemorySet::from_elf(elf_data);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        self.inner.lock().memory = memory_set;
        self.inner.lock().trap_ctx = trap_ctx_ppn;
        let trap_ctx = trap_ctx_ppn.get_mut();
        *trap_ctx = TrapContext::app_init_context(
            user_sepc,
            user_sp,
            KERNEL_SPACE.get().token(),
            self.kernel_stack.top(),
            trap_handler as usize,
        );
    }

    pub fn task_ctx_ptr(&self) -> *const TaskContext {
        &self.inner.lock().task_ctx
    }

    pub fn trap_ctx(&self) -> &'static mut TrapContext {
        self.inner.lock().trap_ctx.get_mut()
    }

    pub fn user_token(&self) -> usize {
        self.inner.lock().memory.token()
    }
}