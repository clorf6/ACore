pub mod address;
pub mod buddy_allocator;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod linked_list;
pub mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}
