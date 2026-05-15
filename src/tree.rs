use std::alloc::{Layout, alloc, dealloc};
use std::ptr;

struct Node<T: Ord> {
    left: *mut Node<T>,
    right: *mut Node<T>,
    data: T,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn from(data: T) -> Self {
        Self {
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            data,
        }
    }
}

pub struct Tree<T: Ord> {
    root: *mut Node<T>,
    len: usize,
}

impl<T> Tree<T>
where
    T: Ord,
{
    pub fn from(data: T) -> Self {
        let layout = Layout::new::<Node<T>>();
        let ptr = unsafe { alloc(layout) as *mut Node<T> };
        let node_struct = Node::from(data);

        unsafe {
            ptr.write(node_struct);
        }

        Self { root: ptr, len: 1 }
    }
    pub fn insert(&mut self, data: T) {
        insert(ptr::addr_of_mut!(self.root), data);
        self.len += 1;
    }

    pub fn contains(&self, data: T) -> bool {
        contains(self.root, data)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn insert<T: Ord>(node: *mut *mut Node<T>, data: T) {
    if unsafe { (*node).is_null() } {
        let layout = Layout::new::<Node<T>>();
        let new_node = unsafe { alloc(layout) as *mut Node<T> };
        let node_struct = Node::from(data);

        unsafe {
            new_node.write(node_struct);
        };

        unsafe {
            *node = new_node;
        };

        return;
    }

    if unsafe { data < (**node).data } {
        unsafe {
            insert(&mut (**node).left, data);
        };
    } else {
        unsafe {
            insert(&mut (**node).right, data);
        };
    }
}

fn contains<T: Ord>(node: *mut Node<T>, data: T) -> bool {
    if node.is_null() {
        return false;
    }
    if unsafe { (*node).data == data } {
        return true;
    }

    if unsafe { data < (*node).data } {
        unsafe { contains((*node).left, data) }
    } else {
        unsafe { contains((*node).right, data) }
    }
}
