use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ops::{Index, IndexMut};
use std::ptr;

#[macro_export]
macro_rules! arr {
    ($($x: expr), *) => {
        {
            let mut arr = Array::new();
            $(arr.push($x);)*
            arr
        }
    };
}

pub struct Array<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        let capacity = 4;
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };

        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len >= self.capacity {
            // Update len and cap
            self.capacity *= 2;

            let layout = Layout::array::<T>(self.capacity).unwrap();
            let new_size = self.capacity * std::mem::size_of::<T>();
            self.ptr = unsafe { realloc(self.ptr as *mut u8, layout, new_size) as *mut T };

            unsafe {
                let new_ptr = self.ptr.add(self.len);
                *new_ptr = value;
            };
        } else {
            unsafe {
                ptr::write(self.ptr.add(self.len), value);
            }
        }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // Index out of bounds
        if index >= self.len {
            return None;
        }

        let new_ptr = unsafe { self.ptr.add(index) };
        return Some(unsafe { &*new_ptr });
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        let new_ptr = unsafe { self.ptr.add(index) };
        return Some(unsafe { &mut *new_ptr });
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.len));
            let layout = Layout::array::<T>(self.capacity).unwrap();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}
