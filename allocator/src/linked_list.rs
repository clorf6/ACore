use core::ptr::null_mut;
use core::ptr::write;

pub struct Node {
    next: *mut Node,
}

#[derive(Clone, Copy, Debug)]
pub struct LinkedList {
    head: *mut Node,
}

unsafe impl Send for LinkedList {}

impl LinkedList {
    pub const fn new() -> Self {
        LinkedList { head: null_mut() }
    }

    pub unsafe fn push_front(&mut self, node: usize) {
        write(node as *mut Node, Node { next: self.head });
        self.head = node as *mut Node;
    }

    pub fn pop_front(&mut self) -> Option<*mut Node> {
        if self.head.is_null() {
            return None;
        }
        let node = self.head;
        self.head = unsafe { (*node).next };
        Some(node)
    }

    pub fn pop(&mut self, node: *mut Node) {
        let nex = unsafe { (*node).next };
        unsafe { (*node).next = (*nex).next; }
    }

    pub fn find_pre(&self, node: usize) -> *mut Node {
        let mut p = self.head;
        while !p.is_null() {
            let nex = unsafe { (*p).next };
            if nex as usize == node {
                return p;
            }
            p = nex;
        }
        null_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

}

