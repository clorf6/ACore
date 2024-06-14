#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::{exec, fork};
#[no_mangle]
pub fn main() -> i32 {
    println!("[User] Init process started!");
    if fork() == 0 {
        println!("[User] fork child!");
        exec("hello\0");
    } else {
        println!("[User] fork father!");
    }
    0
}
