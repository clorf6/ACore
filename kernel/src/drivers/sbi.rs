#![allow(unused)]

use core::sync::atomic::{AtomicPtr, Ordering};
use core::fmt::{self, Write};
use crate::config::VIRT_UART;
use lazy_static::lazy_static;
use sync::UPSafeCell;

#[derive(Debug)]
pub struct UartPort {
    dbr: AtomicPtr<u8>, // Data Buffer Register
    ier: AtomicPtr<u8>, // Interrupt Enable Register
    fcr: AtomicPtr<u8>, // FIFO Control Register
    lcr: AtomicPtr<u8>, // Line Control Register
    mcr: AtomicPtr<u8>, // Modem Control Register
    lsr: AtomicPtr<u8>, // Line Status Register
}

lazy_static! {
    pub static ref UART: UPSafeCell<UartPort> = unsafe { UPSafeCell::new({
        UartPort::new(VIRT_UART)
    }) };
}

pub fn init_uart() {
    UART.lock().start();
}

impl UartPort {

    pub unsafe fn new(base: usize) -> Self {
        Self {
            dbr: AtomicPtr::new(base as *mut u8),
            ier: AtomicPtr::new((base + 1) as *mut u8),
            fcr: AtomicPtr::new((base + 2) as *mut u8),
            lcr: AtomicPtr::new((base + 3) as *mut u8),
            mcr: AtomicPtr::new((base + 4) as *mut u8),
            lsr: AtomicPtr::new((base + 5) as *mut u8),
        }
    }

    pub fn start(&mut self) {
        let dbr = self.dbr.load(Ordering::Relaxed);
        let ier = self.ier.load(Ordering::Relaxed);
        let fcr = self.fcr.load(Ordering::Relaxed);
        let lcr = self.lcr.load(Ordering::Relaxed);
        let mcr = self.mcr.load(Ordering::Relaxed);
        unsafe {
            ier.write(0x00); // Disable all interrupts
            lcr.write(0x80); // Enable DLAB
            dbr.write(0x03); 
            ier.write(0x00); // Set DLL, DLM to 38400 baud
            lcr.write(0x03); // Disable DLAB, set data length to 8 bits
            fcr.write(0xC7); // Enable FIFO, clear them, with 14-byte threshold
            mcr.write(0x0B); // IRQs enabled, RTS/DSR set
            ier.write(0x01); // Enable interrupts
        }
    }

    pub fn send(&mut self, data: u8) {
        let dbr = self.dbr.load(Ordering::Relaxed);
        unsafe {
            match data {
                8 | 0x7F => {
                    while self.lsr.load(Ordering::Relaxed).read() & 0x20 == 0 { core::hint::spin_loop(); }
                    dbr.write(8);
                    while self.lsr.load(Ordering::Relaxed).read() & 0x20 == 0 { core::hint::spin_loop(); }
                    dbr.write(b' ');
                    while self.lsr.load(Ordering::Relaxed).read() & 0x20 == 0 { core::hint::spin_loop(); }
                    dbr.write(8);
                }
                _ => {
                    while self.lsr.load(Ordering::Relaxed).read() & 0x20 == 0 { core::hint::spin_loop(); }
                    dbr.write(data);
                }
            }
        }
    }

    pub fn receive(&mut self) -> u8 {
        let dbr = self.dbr.load(Ordering::Relaxed);
        unsafe {
            while self.lsr.load(Ordering::Relaxed).read() & 0x01 == 0 { core::hint::spin_loop(); }
            dbr.read()
        }
    }

    pub fn read_char(&mut self) -> u8 {
        self.receive()
    }

    pub fn write_char(&mut self, c: u8) {
        self.send(c);
    }
}

impl Write for UartPort {
    #[allow(deprecated)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.send(c);
        }
        Ok(())
    }
}
