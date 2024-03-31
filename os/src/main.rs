#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use core::arch::asm;
use crate::sbi::UART;
// use crate::config::TIME_PERIOD;
use riscv::register::*;

#[macro_use]
mod exception;
mod console;
mod config;
mod sync;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

pub fn init_time() {
    // let mtime = 0x0200bff8 as *const usize;
    // let time = mtime.read_volatile();
    // let mtimecmp = 0x02004000 as *mut usize;
    // *mtimecmp = time;
    // ToDo
}

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
    unsafe { asm!("mret"); }
}

/// the rust entry-point of os
#[no_mangle]
pub unsafe fn rust_main() -> ! {
    UART.get().start(); 
    clear_bss();
    rust_start();
    println!("Hello, world!!!");
    panic!("It should shutdown!");
}