//! Because rust does not have fixed length dynamically allocated array, I will
//! be using Vec instead. Although Vec is already a dynamic array, I will be
//! pretending that it is a fixed length array for the purpose of this
//! assignemnt.
//!
//! Note if Vec is initialized with vec![0; len] and push is never called, then
//! it will never allocate more memory than requested and will behave like a
//! fixed length array in all aspect.
//! According to https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees

use std::mem;

use crate::{Element, Key};

pub struct DynamicArray {
    arr: Vec<Element>,
    len: usize,
}

impl DynamicArray {
    pub fn new() -> Self {
        Self {
            len: 0,
            arr: vec![(0, 0)],
        }
    }

    pub fn insert(&mut self, x: Element) {
        if self.len == self.arr.len() {
            self.resize(self.arr.len() * 2);
        }
        self.arr[self.len] = x;
        self.len += 1;
    }

    fn resize(&mut self, new_len: usize) {
        assert!(new_len >= self.len);

        let old = mem::replace(&mut self.arr, vec![(0, 0); new_len]);
        for (i, x) in old.into_iter().enumerate().take(new_len) {
            self.arr[i] = x;
        }
    }

    pub fn delete(&mut self, k: Key) {
        for i in 0..self.len {
            if self.arr[i].1 == k {
                self.arr.swap(self.len - 1, i);
                self.len -= 1; // remove last element
                if self.len * 4 < self.arr.len() {
                    self.resize(self.len / 2);
                }
                break;
            }
        }
    }

    /// Rust does not have NULL so I have to replace it with Option::None
    pub fn search(&self, k: Key) -> Option<Element> {
        for i in 0..self.len {
            if self.arr[i].1 == k {
                return Some(self.arr[i]);
            }
        }
        None
    }
}
