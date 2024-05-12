pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_area;
pub mod page_table;
pub mod range;

pub use page_table::{translated_byte_buffer};
pub use address::{PhysPageNum, VirtAddr, VirtPageNum};
pub use memory_area::{MapPermission, KERNEL_SPACE, MemorySet};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.get().activate();
    memory_area::remap_test();
}
