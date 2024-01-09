use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Trie {
    is_end: bool,
    children: Rc<RefCell<HashMap<u8, Trie>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self {
            is_end: false,
            children: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn new_node(is_end: bool) -> Self {
        Self {
            is_end,
            children: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, word: String) {
        let word = word.as_bytes();
        let mut p = self.children.clone();
        for i in 0..word.len() - 1 {
            let children;
            let mut temp = p.borrow_mut();
            if let Some(child) = temp.get(&word[i]) {
                children = child.children.clone();
            } else {
                let child = Self::new_node(false);
                children = child.children.clone();
                temp.insert(word[i], child);
            }
            drop(temp);
            p = children;
        }
        let mut p = p.borrow_mut();
        if let Some(child) = p.get_mut(&word[word.len() - 1]) {
            child.is_end = true;
        } else {
            let child = Self::new_node(true);
            p.insert(word[word.len() - 1], child);
        }
    }

    pub fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        let mut is_end = false;
        let mut p = self.children.clone();
        for c in word {
            let children;
            if let Some(child) = p.borrow().get(c) {
                is_end = child.is_end;
                children = child.children.clone();
            } else {
                return false;
            }
            p = children;
        }
        is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let prefix = prefix.as_bytes();
        let mut p = self.children.clone();
        for c in prefix {
            let children;
            if let Some(child) = p.borrow().get(c) {
                children = child.children.clone();
            } else {
                return false;
            }
            p = children;
        }
        true
    }
}
