#![no_std]
#![no_main]

extern crate user;
use user::{sleep, set_priority};

#[no_mangle]
pub fn main() ->isize{
    let res = set_priority(160);
    sleep(3000);
    res
}