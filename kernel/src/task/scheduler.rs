use super::task::Task;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use sync::UPSafeCell;

lazy_static! {
    pub static ref MAX_STRIDE: UPSafeCell<usize> = UPSafeCell::new(0);
}

pub struct ScheduleUnit {
    pub task: Arc<Task>,
    pub pass: usize,
}

impl Eq for ScheduleUnit {}

impl PartialEq for ScheduleUnit {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass
    }
}

impl Ord for ScheduleUnit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.pass > other.pass {
            if self.pass - other.pass <= usize::MAX / 2 {
                core::cmp::Ordering::Less
            } else {
                core::cmp::Ordering::Greater
            }
        } else if self.pass < other.pass {
            if other.pass - self.pass <= usize::MAX / 2 {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Less
            }
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for ScheduleUnit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

