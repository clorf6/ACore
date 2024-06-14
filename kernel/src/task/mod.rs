mod stack;
mod task;
mod switch;
mod manager;
mod processor;

pub use stack::{KernelStack};
pub use switch::__switch;
pub use task::{Task, TaskStatus, TaskContext};
pub use manager::{push_back, push_front, get_front_task, init_tasks, task_num, MANAGERTASK, set_server};
pub use processor::{get_cur_task, take_cur_task, schedule, run_tasks};

pub fn suspend_and_yield() {
    schedule(TaskStatus::Ready, true);
}

pub fn exit_and_yield() {
    schedule(TaskStatus::Zombie, false);
}