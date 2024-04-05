pub mod buddy_allocator;
pub mod linked_list;
pub mod heap_allocator;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}
