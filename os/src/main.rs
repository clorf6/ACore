#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use crate::drivers::sbi::init_uart;
use crate::drivers::sbi::UART;
use core::arch::asm;
use core::arch::global_asm;
// use crate::config::TIME_PERIOD;
use riscv::register::*;
extern crate alloc;
extern crate bitflags;

#[macro_use]
mod drivers;
mod config;
mod console;
mod exception;
mod mm;
mod sync;

global_asm!(include_str!("entry.asm"));

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

pub fn init_time() {
    // let mtime = 0x0200bff8 as *const usize;
    // let time = mtime.read_volatile();
    // let mtimecmp = 0x02004000 as *mut usize;
    // *mtimecmp = time;
    // ToDo
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
    // init_time();
    unsafe {
        asm!("mret", options(noreturn));
    }
}

#[no_mangle]
pub unsafe fn rust_main() -> ! {
    init_uart();
    clear_bss();
    mm::init();
    println!("Hello, world!!!");
    panic!("It should shutdown!");
}
