use self::memory_area::KERNEL_SPACE;

pub mod address;
pub mod buddy_allocator;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod linked_list;
pub mod memory_area;
pub mod page_table;
pub mod range;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.get().activate();
}
