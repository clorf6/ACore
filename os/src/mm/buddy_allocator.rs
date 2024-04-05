use super::linked_list::LinkedList;
use alloc::alloc::Layout;
use core::alloc::GlobalAlloc;
use core::ptr::null_mut;
use crate::config::BUDDY_ALLOCATOR_LEVEL;
use core::cmp::{min, max};
use core::mem::size_of;
use spin::Mutex;

pub struct BuddyAllocator {
    pub allocator: Mutex<BuddyAllocatorInner>,
}

impl BuddyAllocator {
    pub const fn new(minimum: usize) -> Self {
        Self { allocator: Mutex::new(BuddyAllocatorInner::new(minimum)), }
    }

    pub fn lock(&self) -> spin::MutexGuard<BuddyAllocatorInner> {
        self.allocator.lock()
    }

    pub unsafe fn add(&self, begin: usize, end: usize) {
        self.allocator.lock().add(begin, end);
    }
}

unsafe impl GlobalAlloc for BuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}

pub struct BuddyAllocatorInner {
    free_lists: [LinkedList; BUDDY_ALLOCATOR_LEVEL],
    total: usize,
    minimum: usize,
    used: usize,
    need: usize,
}

impl BuddyAllocatorInner {
    pub const fn new(minimum: usize) -> Self {
        Self {
            free_lists: [LinkedList::new(); BUDDY_ALLOCATOR_LEVEL],
            total: 0,
            minimum: if minimum < size_of::<usize>() { size_of::<usize>() } else { minimum },
            used: 0,
            need: 0,
        }
    }

    pub fn add(&mut self, mut begin: usize, mut end: usize) {
        begin = (begin + self.minimum - 1) & (!self.minimum + 1);
        end = end & (!self.minimum + 1);
        let mut size = end - begin;
        self.total += size;
        while size > 0 {
            let level = size.trailing_zeros() as usize;
            unsafe { self.free_lists[level].push_front(begin); }
            size -= 1 << level;
            begin += 1 << level;
        }
    }

    fn split(&mut self, big: usize, small: usize) {
        for i in (small..big).rev() {
            let now = self.free_lists[i + 1].pop_front().expect("[buddy allocator] Free list is empty.");
            unsafe {
                self.free_lists[i].push_front(now as usize + (1 << i));
                self.free_lists[i].push_front(now as usize);
            }
        }
    }

    fn merge(&mut self, begin: usize, ptr: *mut u8) {
        let mut now = ptr as usize;
        for i in begin..self.free_lists.len() {
            let buddy = now ^ (1 << i);
            let node = self.free_lists[i].find_pre(buddy);
            if !node.is_null() {
                self.free_lists[i].pop(node);
                now = min(now, buddy);
            } else {
                unsafe { self.free_lists[i].push_front(now); }
                break;
            }
        }
    }

    fn calc_size(&self, layout: &Layout) -> usize {
        max(
            max(layout.align(), self.minimum),
            layout.size().next_power_of_two()
        )
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = self.calc_size(&layout);
        let level = size.trailing_zeros() as usize;
        for i in level..self.free_lists.len() {
            if !self.free_lists[i].is_empty() {
                self.split(i, level);
                let res = self.free_lists[level].pop_front().expect("[buddy allocator] Free list is empty.");
                self.used += size;
                self.need += layout.size();
                return res as *mut u8;
            }
        }
        null_mut()
    }

    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = self.calc_size(&layout);
        let level = size.trailing_zeros() as usize;
        self.merge(level, ptr);
    }
}

