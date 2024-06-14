#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::fork;
#[no_mangle]
pub fn main() -> i32 {
    println!("[User] Hello!");
    0
}
