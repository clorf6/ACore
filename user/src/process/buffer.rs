const BUFFER: usize = 0xffffffffffffd000;
const BUFFER_SIZE: usize = 0x1000;

use core::cmp::min;

pub fn get_buffer(len: usize) -> &'static mut [usize] {
    if len > BUFFER_SIZE {
        panic!("[user] Buffer overflow");
    }
    unsafe { core::slice::from_raw_parts_mut(BUFFER as *mut usize, len) }
}

pub fn write_to_buffer(data: &[usize]) {
    let buffer = get_buffer(min(data.len(), BUFFER_SIZE));
    for (i, &byte) in data.iter().enumerate() {
        buffer[i] = byte;
    }
}

pub fn read_from_buffer<const N: usize>() -> [usize; N] {
    let buffer = get_buffer(N);
    let mut array = [0; N];
    array.copy_from_slice(&buffer[..N]);
    array
}

pub fn buffer_test() {
    let data: [usize; 4] = read_from_buffer();
    for byte in data.iter() {
        println!("[User] Read from buffer: {}", byte);
    }
}
