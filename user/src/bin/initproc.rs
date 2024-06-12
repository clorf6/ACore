#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[no_mangle]
pub fn main() -> i32 {
    println!("[User] Hello, world");
    user::process::init_processes();
    user::process::buffer_test();
    0
}
