use crate::config::{FRAME_END, PAGE_SIZE};
use crate::println;
use core::cmp::min;
pub fn buffer_position(pid: usize) -> (usize, usize) {
    // [bottom, top)
    let bottom = FRAME_END + pid * PAGE_SIZE;
    let top = bottom + PAGE_SIZE;
    (bottom, top)
}
fn get_shared_buffer(pid: usize, len: usize) -> &'static mut [usize] {
    if len > PAGE_SIZE {
        panic!("[kernel] Buffer overflow");
    }
    let (addr, _) = buffer_position(pid);
    unsafe { core::slice::from_raw_parts_mut(addr as *mut usize, len) }
}
pub fn write_to_buffer(data: &[usize], pid: usize) {
    let buffer = get_shared_buffer(pid, min(data.len(), PAGE_SIZE));
    for (i, &byte) in data.iter().enumerate() {
        buffer[i] = byte;
    }
}
pub fn read_from_buffer<const N: usize>(pid: usize) -> [usize; N] {
    let buffer = get_shared_buffer(pid, N);
    let mut array = [0; N];
    array.copy_from_slice(&buffer[..N]);
    array
}

#[allow(unused)]
pub fn buffer_test() {
    write_to_buffer(&[3, 5, 3, 6], 0);
    println!("[kernel] Successfully wrote to buffer");
}
