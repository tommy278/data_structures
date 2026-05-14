use std::alloc::{Layout, alloc, dealloc};
use std::ops::{Index, IndexMut};
use std::ptr;

#[macro_export]
macro_rules! list {
    ($($x: expr), *) => {
        {
            let mut ll = LinkedList::new();
            $(ll.push_back($x);)*
            ll
        }
    };
}

struct Node<T> {
    data: T,
    prev_ptr: *mut Node<T>,
    next_ptr: *mut Node<T>,
}

impl<T> Node<T> {
    fn from_val(val: T) -> Self {
        Self {
            data: val,
            prev_ptr: ptr::null_mut(),
            next_ptr: ptr::null_mut(),
        }
    }
}

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn from_val(val: T) -> Self {
        let layout = Layout::new::<Node<T>>();

        let ptr = unsafe { alloc(layout) as *mut Node<T> };
        let node = Node::from_val(val);

        unsafe {
            ptr.write(node);
        }

        Self {
            head: ptr,
            tail: ptr,
            len: 1,
        }
    }

    pub fn get(&self, mut index: usize) -> Option<&T> {
        if index == 0 && !(self.head).is_null() {
            // If getting first data, return the data in head
            return Some(unsafe { &(*self.head).data });
        } else if index == self.len.saturating_sub(1) && !(self.tail).is_null() {
            // If getting last data, return data in tail
            return Some(unsafe { &(*self.tail).data });
        } else {
            // Index out of bound
            if index >= self.len {
                return None;
            }

            // Iterate through each pointer till the index is reached

            let mut ptr = self.head;

            while index > 0 {
                let next_ptr = unsafe { (*ptr).next_ptr };
                index -= 1;
                ptr = next_ptr;
            }

            if ptr.is_null() {
                println!("76");
                return None;
            }

            return Some(unsafe { &(*ptr).data });
        }
    }

    // Simply a mutable get
    fn get_mut(&self, mut index: usize) -> Option<&mut T> {
        if index == 0 && !(self.head).is_null() {
            return Some(unsafe { &mut (*self.head).data });
        } else if index == self.len.saturating_sub(1) && !(self.tail).is_null() {
            return Some(unsafe { &mut (*self.tail).data });
        } else {
            if index >= self.len {
                return None;
            }

            let mut ptr = self.head;

            while index > 0 {
                let next_ptr = unsafe { (*ptr).next_ptr };
                index -= 1;
                ptr = next_ptr;
            }

            if ptr.is_null() {
                println!("76");
                return None;
            }

            return Some(unsafe { &mut (*ptr).data });
        }
    }

    pub fn push_back(&mut self, val: T) {
        if self.tail.is_null() {
            let list = LinkedList::from_val(val);
            *self = list;
        } else {
            // Allocate data for new node
            let layout = Layout::new::<Node<T>>();
            let ptr = unsafe { alloc(layout) as *mut Node<T> };

            let mut new_node = Node::from_val(val);
            // Write new val into allocate ptr

            new_node.prev_ptr = self.tail;

            unsafe { ptr.write(new_node) };

            // Update shared_ptr to the new value
            unsafe {
                (*self.tail).next_ptr = ptr;
            }

            // Update the tail to new allocated data
            self.tail = ptr;

            self.len += 1;
        }
    }

    // Inverse of push back
    pub fn push_front(&mut self, val: T) {
        if self.tail.is_null() {
            let list = LinkedList::from_val(val);
            *self = list;
        } else {
            let layout = Layout::new::<Node<T>>();
            let ptr = unsafe { alloc(layout) as *mut Node<T> };
            let mut new_node = Node::from_val(val);
            new_node.next_ptr = self.head;

            unsafe {
                ptr.write(new_node);
                (*self.head).prev_ptr = ptr;
            };

            self.head = ptr;
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // Empty list
        if self.is_empty() {
            return None;
        }

        // List is empty, update head accordingly
        if self.tail.is_null() {
            self.head = ptr::null_mut();
            return None;
        }

        // Store tail's previous pointer and store tail's value
        let layout = Layout::new::<Node<T>>();
        let new_ptr = unsafe { (*self.tail).prev_ptr };
        let value = unsafe { ptr::read(&(*self.tail).data) };

        unsafe {
            dealloc(self.tail as *mut u8, layout);
        }

        // Update tail to previous pointer
        self.tail = new_ptr;

        if !self.is_empty() && !self.tail.is_null() {
            unsafe { (*self.tail).next_ptr = ptr::null_mut() };
        }

        self.len -= 1;

        return Some(value);
    }

    // Inverse of pop_back

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.head.is_null() {
            self.tail = ptr::null_mut();
            return None;
        }

        let layout = Layout::new::<Node<T>>();
        let new_ptr = unsafe { (*self.head).next_ptr };
        let value = unsafe { ptr::read(&(*self.head).data) };

        unsafe {
            dealloc(self.head as *mut u8, layout);
        }

        self.head = new_ptr;

        if !self.is_empty() && !self.head.is_null() {
            unsafe { (*self.head).prev_ptr = ptr::null_mut() };
        }

        self.len -= 1;

        return Some(value);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn last(&self) -> Option<&T> {
        self.get(self.len - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for LinkedList<T> {
    /* Simply drop all values, they will be deallocated and dropped
    by pop */
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop_back();
        }
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
