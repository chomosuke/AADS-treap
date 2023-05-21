use std::mem;

use crate::{random::random_10_7, Element, Key, ID};

struct Node {
    x: Element,
    priority: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

enum RotateResult {
    Same,
    Left,
    Right,
}

impl Node {
    fn rotate(&mut self) -> RotateResult {
        if let Some(left) = &self.left {
            if self.get_priority() > left.get_priority() {
                let mut left = *self.left.take().unwrap();
                mem::swap(&mut left, self);
                let mut prev_self = left;
                prev_self.left = self.right.take();
                self.right = Some(Box::new(prev_self));
                return RotateResult::Right;
            }
        }
        if let Some(right) = &self.right {
            if self.get_priority() > right.get_priority() {
                // disconnect right with self
                let mut right = *self.right.take().unwrap();
                // self is now right
                mem::swap(&mut right, self);
                let mut prev_self = right;
                // self's left is prev_self's right
                prev_self.right = self.left.take();
                // prev_self is now self's left
                self.left = Some(Box::new(prev_self));
                return RotateResult::Left;
            }
        }
        RotateResult::Same
    }

    fn get_priority(&self) -> (u32, Key, ID) {
        (self.priority, self.x.1, self.x.0)
    }

    fn insert(&mut self, x: Element) {
        if (self.x.1, self.x.0) < (x.1, x.0) {
            if let Some(left) = &mut self.left {
                left.insert(x);
            } else {
                self.left = Some(Box::new(Node {
                    x,
                    priority: random_10_7(),
                    left: None,
                    right: None,
                }));
            }
        } else {
            if let Some(left) = &mut self.left {
                left.insert(x);
            } else {
                self.left = Some(Box::new(Node {
                    x,
                    priority: random_10_7(),
                    left: None,
                    right: None,
                }));
            }
        }
        self.rotate();
    }
}

pub struct Treap {
    root: Option<Box<Node>>,
}

impl Treap {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, x: Element) {
        if let Some(root) = &mut self.root {
            root.insert(x);
        } else {
            self.root = Some(Box::new(Node {
                x,
                priority: random_10_7(),
                left: None,
                right: None,
            }));
        }
    }

    pub fn delete(&mut self, k: Key) {
        let mut node = &mut self.root;
        let mut node = loop {
            if node.is_some() {
                if node.as_ref().unwrap().x.1 < k {
                    node = &mut node.as_mut().unwrap().left;
                } else if node.as_ref().unwrap().x.1 > k {
                    node = &mut node.as_mut().unwrap().right;
                } else {
                    break node;
                }
            } else {
                return;
            }
        };
        // set priority to inf
        node.as_mut().unwrap().priority = u32::MAX;
        loop {
            match node.as_mut().unwrap().rotate() {
                RotateResult::Same => break,
                RotateResult::Left => node = &mut node.as_mut().unwrap().left,
                RotateResult::Right => node = &mut node.as_mut().unwrap().right,
            }
        }
        *node = None;
    }

    /// Rust does not have NULL so I have to replace it with Option::None
    pub fn search(&self, k: Key) -> Option<Element> {
        let mut node = &self.root;
        while let Some(n) = node {
            if n.x.1 < k {
                node = &n.left;
            } else if n.x.1 > k {
                node = &n.right;
            } else {
                return Some(n.x);
            }
        }
        None
    }
}
