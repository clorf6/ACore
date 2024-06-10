use core::arch::global_asm;
use crate::config::{CPU_NUMBER, MTIMECMP_ADDR, MTIME_ADDR, TIME_PERIOD};
use riscv::register::{mhartid, mie, mscratch, mstatus, mtvec};

#[link_section = ".bss.stack"]
#[no_mangle]
static mut TIME_SCRATCH: [[usize; 5]; CPU_NUMBER] = [[0; 5]; CPU_NUMBER];

global_asm!(include_str!("trap.S"));

pub fn get_time() -> usize {
    unsafe { *(MTIME_ADDR as *const usize) }
}

pub fn set_time(addr: *mut usize, time: usize) {
    unsafe { 
        *addr = time;
    }
}

#[no_mangle]
pub unsafe fn init_time() {
    let hartid = mhartid::read();
    let addr = MTIMECMP_ADDR + hartid * 8;
    set_time(addr as *mut usize, get_time() + TIME_PERIOD);
    let scratch = &mut TIME_SCRATCH[hartid];
    scratch[0] = addr;
    scratch[1] = TIME_PERIOD;
    mscratch::write(scratch as *mut _ as usize);
    extern "C" {
        fn __timetrap();
    }
    mtvec::write(__timetrap as usize, mtvec::TrapMode::Direct);
    mstatus::set_mie();
    mie::set_mtimer();
}