pub use address::{PhysPageNum, VirtAddr};
pub use buffer::buffer_position;
pub use memory_area::{KERNEL_SPACE, MapPermission, MemorySet};
pub use page_table::translated_byte_buffer;

pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_area;
pub mod page_table;
pub mod range;
pub mod buffer;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.get().activate();
    //heap_allocator::heap_test();
    // memory_area::remap_test();
}
