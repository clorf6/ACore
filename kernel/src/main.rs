#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate bitflags;

use core::arch::asm;
use core::arch::global_asm;

use lazy_static::*;
use riscv::register::*;

use crate::drivers::sbi::init_uart;
use crate::loader::list_apps;
use log::*;
use crate::console::{getchar, putchar};

#[macro_use]
mod drivers;
mod config;
mod exception;
mod mm;
mod loader;
mod time;
pub mod logging;
pub mod console;
pub mod syscall;
pub mod trap;
pub mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub unsafe fn rust_start() {
    mstatus::set_mpp(mstatus::MPP::Supervisor);
    mepc::write(rust_main as usize);
    satp::write(0);
    unsafe {
        asm!(
            "csrw mideleg, {medeleg}",
            "csrw medeleg, {mideleg}",
            medeleg = in(reg) !0,
            mideleg = in(reg) !0,
        );
    }
    sie::set_sext();
    sie::set_stimer();
    sie::set_ssoft();
    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);
    time::init_time();
    unsafe {
        asm!("mret", options(noreturn));
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    init_uart();
    clear_bss();
    logging::init();
    mm::init();
    trap::init();
    list_apps();
    task::init_tasks();
    task::run_tasks();
    panic!("It should shutdown!");
}
