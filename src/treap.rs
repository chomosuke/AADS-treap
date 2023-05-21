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
            if let Some(right) = &mut self.right {
                right.insert(x);
            } else {
                self.right = Some(Box::new(Node {
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
            if k < n.x.1 {
                node = &n.left;
            } else if n.x.1 < k {
                node = &n.right;
            } else {
                return Some(n.x);
            }
        }
        None
    }

    pub fn get_depths(&self) -> Vec<usize> {
        let mut depths = Vec::new();
        fn traverse(node: &Box<Node>, depth: usize, depths: &mut Vec<usize>) {
            depths.push(depth);
            if let Some(node) = &node.right {
                traverse(node, depth + 1, depths);
            }
            if let Some(node) = &node.left {
                traverse(node, depth + 1, depths);
            }
        }
        if let Some(root) = &self.root {
            traverse(root, 0, &mut depths);
        }
        depths
    }
}

#[test]
fn test_node() {
    let mut node = Node {
        x: (0, 1),
        priority: 3,
        left: Some(Box::new(Node {
            x: (0, 1),
            priority: 2,
            left: Some(Box::new(Node {
                x: (0, 1),
                priority: 5,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                x: (0, 1),
                priority: 6,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            x: (0, 1),
            priority: 4,
            left: None,
            right: None,
        })),
    };
    assert!(matches!(node.rotate(), RotateResult::Right));
    assert_eq!(node.priority, 2);
    assert_eq!(node.left.as_ref().unwrap().priority, 5);
    assert_eq!(node.right.as_ref().unwrap().priority, 3);
    assert_eq!(
        node.right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .priority,
        4
    );
    assert_eq!(
        node.right.as_ref().unwrap().left.as_ref().unwrap().priority,
        6
    );

    node.left.as_mut().unwrap().priority = 1;
    node.rotate();
    assert_eq!(node.priority, 1);
    assert_eq!(node.right.as_ref().unwrap().priority, 2);
    assert_eq!(
        node.right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .priority,
        3
    );
    assert_eq!(
        node.right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .priority,
        4
    );
    assert_eq!(
        node.right
            .as_ref()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .left
            .as_ref()
            .unwrap()
            .priority,
        6
    );
}
