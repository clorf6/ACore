mod stack;
mod task;
mod switch;
mod manager;
mod processor;

pub use stack::{KernelStack};
pub use switch::__switch;
pub use task::{Task, TaskStatus, TaskContext};
pub use manager::*;
pub use processor::{get_cur_task, user_token, trap_ctx, schedule, run_tasks};

pub fn suspend_and_yield() {
    schedule(true, 0);
}

pub fn exit_and_yield(exit_code: isize) {
    schedule(false, exit_code);
}