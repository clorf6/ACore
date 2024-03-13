use core::fmt::{self, Write};
use crate::sbi::UartPort;

pub fn print(args: fmt::Arguments) {
    let mut uart_port = unsafe { UartPort::new(0x10000000) };
    uart_port.start(); 
    uart_port.write_fmt(args).unwrap();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}