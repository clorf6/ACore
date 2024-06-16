use alloc::alloc::Layout;
use core::alloc::GlobalAlloc;
use core::cell::RefMut;
use core::cmp::max;
use sync::UPSafeCell;

pub struct BuddyAllocator<const NUM: usize, const MINIMUM: usize> {
    pub allocator: UPSafeCell<BuddyAllocatorInner<NUM, MINIMUM>>,
}

impl<const NUM: usize, const MINIMUM: usize> BuddyAllocator<NUM, MINIMUM> {
    pub const fn new() -> Self {
        Self {
            allocator: UPSafeCell::new(BuddyAllocatorInner::new()),
        }
    }

    pub fn lock(&self) -> RefMut<'_, BuddyAllocatorInner<NUM, MINIMUM>> {
        self.allocator.lock()
    }

    pub unsafe fn init(&self, offset: usize) {
            self.allocator.lock().init(offset);
    }

    pub fn used(&self) -> usize {
        self.allocator.lock().used
    }
}

unsafe impl<const NUM: usize, const MINIMUM: usize> GlobalAlloc for BuddyAllocator<NUM, MINIMUM> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}

pub struct BuddyAllocatorInner<const NUM: usize, const MINIMUM: usize> {
    total: usize,
    offset: usize,
    pub used: usize,
    block: [usize; NUM]
}

impl<const NUM: usize, const MINIMUM: usize> BuddyAllocatorInner<NUM, MINIMUM> {
    pub const fn new() -> Self {
        assert!(NUM.is_power_of_two());
        assert!(MINIMUM.is_power_of_two());
        Self {
            total: NUM * (MINIMUM >> 1),
            offset: 0,
            used: 0,
            block: [0; NUM],
        }
    }

    pub fn init(&mut self, offset: usize) {
        self.offset = offset;
        let mut node_size = self.total << 1;
        for i in 0..NUM - 1 {
            if (i + 1).is_power_of_two() {
                node_size >>= 1;
            }
            self.block[i] = node_size;
        }
    }

    fn calc_size(&self, layout: &Layout) -> usize {
        max(
            max(layout.align(), MINIMUM),
            layout.size().next_power_of_two(),
        )
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = self.calc_size(&layout);
        assert!(size.is_power_of_two());
        let mut node_size = self.total;
        let mut idx = 0;
        while node_size != size {
            if idx >= self.block.len() || (idx as isize) < 0 {
                panic!("[buddy allocator] Index out of bounds.");
            }
            if self.block[(idx << 1) + 1] >= size {
                idx = (idx << 1) + 1;
            } else {
                idx = (idx << 1) + 2;
            }
            node_size >>= 1;
        }
        if self.block[idx] < size {
            panic!("[buddy allocator] Size not enough. {}, {}, {}, {}", self.block[idx], size, self.used, self.total);
        };
        self.used += self.block[idx];
        self.block[idx] = 0;
        let offset = (idx + 1) * node_size - self.total;
        while idx != 0 {
            idx = ((idx + 1) >> 1) - 1;
            if idx >= self.block.len() || (idx as isize) < 0 {
                panic!("[buddy allocator] Index out of bounds.");
            }
            self.block[idx] = max(self.block[(idx << 1) + 1], self.block[(idx << 1) + 2]);
        }
        (self.offset + offset) as *mut u8
    }

    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = self.calc_size(&layout);
        let addr = ptr as usize - self.offset;
        assert!(addr < self.total);
        let mut node_size = MINIMUM;
        let mut idx = addr / MINIMUM + (NUM >> 1) - 1;
        assert!(idx < NUM);
        while self.block[idx] != 0 {
            node_size <<= 1;
            if idx == 0 {
                return ;
            }
            idx = ((idx + 1) >> 1) - 1;
            if idx >= self.block.len() || (idx as isize) < 0 {
                panic!("[buddy allocator] Index out of bounds.");
            }
        }
        if self.block[idx] != 0 || node_size < size {
            panic!("[buddy allocator] No used space")
        }
        self.used -= node_size;
        self.block[idx] = node_size;
        while idx != 0 {
            idx = ((idx + 1) >> 1) - 1;
            if idx >= self.block.len() || (idx as isize) < 0 {
                panic!("[buddy allocator] Index out of bounds.");
            }
            node_size <<= 1;
            let left = self.block[(idx << 1) + 1];
            let right = self.block[(idx << 1) + 2];
            if left + right == node_size {
                self.block[idx] = node_size;
            } else {
                self.block[idx] = max(left, right);
            }
        }
    }
}
