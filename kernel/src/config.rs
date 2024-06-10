pub const VIRT_UART: usize = 0x10000000;
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x800000;
pub const MEMORY_END: usize = 0x88000000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 12;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
pub const MMIO: &[(usize, usize)] = &[
    (0x10000000, 0x9000), 
    (0x100000, 0x2000),
];

pub const TIME_PERIOD: usize = 100000;
pub const MTIME_ADDR: usize = 0x0200bff8;
pub const MTIMECMP_ADDR: usize = 0x02004000;
pub const CPU_NUMBER: usize = 8;