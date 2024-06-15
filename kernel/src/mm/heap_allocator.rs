use allocator::buddy_allocator::BuddyAllocator;
use buddy_system_allocator::LockedHeap;
use crate::println;
use crate::config::{KERNEL_HEAP_SIZE, PAGE_SIZE};

const alloc_minimum: usize = PAGE_SIZE;

const fn get_alloc_num(total: usize, minimum: usize) -> usize {
    (total << 1) / minimum
}

const alloc_num: usize = get_alloc_num(KERNEL_HEAP_SIZE, alloc_minimum);
static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[global_allocator]
//static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
static HEAP_ALLOCATOR: BuddyAllocator<alloc_num, alloc_minimum> = BuddyAllocator::new();

pub fn used() -> usize {
    HEAP_ALLOCATOR.used()
}
pub fn init_heap() {
    unsafe {
        let begin = KERNEL_HEAP_SPACE.as_ptr() as usize;
        HEAP_ALLOCATOR.init(begin);
        //HEAP_ALLOCATOR.lock().init(begin, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[allow(unused)]
pub fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    println!("used {}", used());
    for i in 0..260000 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        assert_eq!(*val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    for i in 0..260000 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        assert_eq!(*val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    println!("used {}", used());
    println!("heap_test passed!");
}
