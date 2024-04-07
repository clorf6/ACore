pub const VIRT_UART: usize = 0x10000000;
pub const KERNEL_HEAP_SIZE: usize = 0x800000;
pub const MEMORY_END: usize = 0x88000000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 12;
// pub const TIME_PERIOD: usize = 1000000;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

pub const MMIO: &[(usize, usize)] = &[
    (0x10000000, 0x9000), 
    (0x100000, 0x2000),
];