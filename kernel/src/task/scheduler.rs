use super::task::Task;
use alloc::sync::Arc;

pub struct ScheduleUnit {
    pub task: Arc<Task>,
    pub stride: isize,
}

impl Eq for ScheduleUnit {}

impl PartialEq for ScheduleUnit {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride
    }
}

impl PartialOrd for ScheduleUnit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.stride.partial_cmp(&other.stride).map(|ord| ord.reverse())
    }
}

impl Ord for ScheduleUnit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.stride.cmp(&other.stride).reverse()
    }
}