mod stack;
mod task;
mod switch;
mod manager;
mod processor;

pub use stack::{KernelStack};
pub use switch::__switch;
pub use task::{Task, TaskStatus, TaskContext};
pub use manager::{push_task, front_task, init_tasks};
pub use processor::{get_cur_task, schedule, run_tasks};

pub fn suspend_and_yield() {
    schedule(TaskStatus::Ready, true, 0);
}

pub fn exit_and_yield(exit_code: isize) {
    schedule(TaskStatus::Zombie, false, exit_code);
}