use super::address::{ PhysPageNum, VirtPageNum, VirtAddr, PhysAddr, PPN_WIDTH };
use super::frame_allocator::*;
use super::range::Step;
use alloc::string::String;
use alloc::collections::BTreeMap;
use bitflags::*;
use alloc::vec::Vec;
use crate::println;
use crate::task::get_cur_task;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: (ppn.0 << 10) | (flags.bits as usize),
        }
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        ((self.bits >> 10) & ((1 << PPN_WIDTH) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
    pub fn is_leaf(&self) -> bool {
        self.readable() | self.writable() | self.executable()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    dir_frames: BTreeMap<PhysPageNum, FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let mut frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        frame.fa = ppn;
        let mut frames = BTreeMap::new();
        frames.insert(frame.ppn, frame);
        PageTable {
            root_ppn: ppn,
            dir_frames: frames,
        }
    }
    fn create_pte(&mut self, vpn: VirtPageNum) -> (&mut PageTableEntry, PhysPageNum) {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                return (pte, ppn);
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.dir_frames.insert(frame.ppn, frame);
                if let Some(frame) = self.dir_frames.get_mut(&ppn) {
                    frame.used += 1;
                    //println!("used+ {}, {}", frame.ppn.0, frame.used);
                }
            }
            ppn = pte.ppn();
        }
        unreachable!();
    }
    fn find_pte(&self, vpn: VirtPageNum) -> (Option<&mut PageTableEntry>, PhysPageNum) {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: (Option<&mut PageTableEntry>, PhysPageNum) = (None, ppn);
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = (Some(pte), ppn);
                break;
            }
            if !pte.is_valid() {
                return (None, ppn);
            }
            ppn = pte.ppn();
        }
        result
    }
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let (pte, last) = self.create_pte(vpn);
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
        if let Some(frame) = self.dir_frames.get_mut(&last) {
            frame.used += 1;
            //println!("used+ {}, {}", frame.ppn.0, frame.used);
        }
    }
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let res = self.find_pte(vpn);
        let pte = res.0.unwrap();
        let mut last = res.1;
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
        let mut frame = self.dir_frames.get_mut(&last).unwrap();
        let mut ppn = frame.ppn;
        last = frame.fa;
        frame.used -= 1;
        println!("used- {}, {}", frame.ppn.0, frame.used);
        while frame.used == 0 && frame.ppn != frame.fa {
            self.dir_frames.remove(&ppn);
            frame = self.dir_frames.get_mut(&last).unwrap();
            ppn = frame.ppn;
            last = frame.fa;
            frame.used -= 1;
            println!("used- {}, {}", frame.ppn.0, frame.used);
        }
    }
    #[allow(unused)]
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << PPN_WIDTH) - 1)),
            dir_frames: BTreeMap::new(),
        }
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).0.map(|pte| *pte)
    }

    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).0.map(|pte| {
            let pa_based: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let ppn: usize = pa_based.into();
            (ppn + offset).into()
        })
    }
    pub fn token(&self) -> usize {
        (8usize << 60) | self.root_ppn.0
    }
}

pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut ret = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            ret.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            ret.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    ret
}

pub fn translated_string(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut ret = String::new();
    let mut va = ptr as usize;
    loop {
        let ch: u8 = *(page_table.translate_va(VirtAddr::from(va)).unwrap().get_mut());
        if ch == 0 {
            break;
        } else {
            ret.push(ch as char);
            va += 1;
        }
    }
    ret
}