#![no_std]

pub mod buddy_allocator;

pub use buddy_allocator::BuddyAllocator;

extern crate alloc;