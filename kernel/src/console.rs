use core::fmt::{self, Write};
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::drivers::sbi::UART;

pub fn putchar(c: u8) {
    UART.get().write_char(c);
}

pub fn print(args: fmt::Arguments) {
    UART.get().write_fmt(args).unwrap();
}

pub fn getchar() -> u8 {
    UART.get().read_char()
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub fn shutdown(reason: bool) -> ! {
    let virt_test = AtomicPtr::new(0x100000 as *mut u32);
    let virt_addr = virt_test.load(Ordering::Relaxed);
    let exit_code = if reason { 0x5555 } else { (1 << 16) | 0x3333 };
    unsafe { virt_addr.write(exit_code); }
    unreachable!()
}