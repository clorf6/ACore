mod pid;
mod process;
mod manager;
mod buffer;

pub use pid::{Pid, alloc_pid};
pub use process::Process;
pub use manager::{INITPROC, init_processes, insert_process, remove_process, find_process, process_num};
pub use buffer::{read_from_buffer, write_to_buffer, buffer_test};