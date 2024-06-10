use alloc::sync::{Weak, Arc};
use alloc::vec::Vec;
use crate::mm::{PhysPageNum, VirtAddr, MemorySet, KERNEL_SPACE};
use spin::Mutex;
use crate::config::TRAP_CONTEXT;
use crate::trap::{TrapContext, trap_return, trap_handler};
use super::{Pid, KernelStack, alloc_pid};
use crate::println;
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
    pub pid: Pid,
    pub kernel_stack: KernelStack,
    pub inner: Mutex<TaskInner>,
}

pub struct TaskInner {
    pub trap_ctx: PhysPageNum,
    pub task_ctx: TaskContext,
    pub task_status: TaskStatus,
    pub memory: MemorySet,
    pub parent: Option<Weak<Task>>,
    pub children: Vec<Arc<Task>>,
    pub exit_code: isize,
}

impl Task {
    pub fn new(elf_data: &[u8]) -> Self {
        let (memory_set, user_sp, user_sepc) = MemorySet::from_elf(elf_data);
        let trap_ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let pid = alloc_pid();
        let kernel_stack = KernelStack::new(pid.0);
        let kernel_stack_top = kernel_stack.top();
        let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
        *trap_ctx = TrapContext::app_init_context(
            user_sepc,
            user_sp,
            KERNEL_SPACE.get().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        Self {
            pid,
            kernel_stack,
            inner: Mutex::new(TaskInner {
                    trap_ctx: trap_ctx_ppn,
                    task_ctx: TaskContext::new(trap_return as usize, kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory: memory_set,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                }),
        }
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