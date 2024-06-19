use core::arch::{asm, global_asm};

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec
};

pub use context::TrapContext;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::println;
use crate::syscall::{sys_exit, syscall};
use crate::task::{get_server, suspend_and_yield, trap_ctx, user_token};

mod context;

global_asm!(include_str!("trampoline.S"));

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler() {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let mut ctx = trap_ctx();
            ctx.sepc += 4;
            //println!("syscall id = {}, pid {}", ctx.x[17], get_cur_task().pid);
            let result = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
            ctx = trap_ctx();
            ctx.x[10] = result;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, trap_ctx().sepc);
            sys_exit(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            sys_exit(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            unsafe { asm!{"csrc sip, 2"}; }
            if !get_server() {
                suspend_and_yield();
            }
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() {
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = user_token();
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "csrr zero, sstatus",
            "fence.i",
            "jr {restore_va}",             
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      
            in("a1") user_satp,        
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap {:?} from kernel, bad addr = {:#x}", scause::read().cause(), stval::read());
}

