use rand::Rng;

use crate::{Element, Key, ID, random::random_10_7};

/// I did not follow the spec on the interface of DataGenerator because I
/// believe the idea behind tuple of (int, key/element) is meant to be enums
/// anyway.
#[derive(Clone, Copy)]
pub enum Action {
    Insertion(Element),
    Deletion(Key),
    Search(Key),
}

pub struct DataGenerator {
    generated: Vec<Option<Key>>,
    next_id: ID,
}

impl DataGenerator {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            generated: Vec::new(),
        }
    }

    fn gen_key(&self) -> Key {
        random_10_7()
    }

    pub fn gen_element(&mut self) -> Element {
        let e = (self.next_id, self.gen_key());
        self.next_id += 1;
        self.generated.push(Some(e.1));
        e
    }

    pub fn gen_insertion(&mut self) -> Action {
        Action::Insertion(self.gen_element())
    }

    pub fn gen_deletion(&mut self) -> Action {
        let id = rand::thread_rng().gen_range(1..self.next_id);
        if let Some(k) = self.generated[(id - 1) as usize] {
            self.generated[(id - 1) as usize] = None;
            Action::Deletion(k)
        } else {
            Action::Deletion(self.gen_key())
        }
    }

    pub fn gen_search(&self) -> Action {
        Action::Search(self.gen_key())
    }
}
