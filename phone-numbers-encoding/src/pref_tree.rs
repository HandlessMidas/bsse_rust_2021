use std::collections::HashMap;

pub struct PrefixTree {
    pub root: Link,
}

type Link = *mut Node;

pub struct Node {
    pub has_values: bool,
    pub values: Vec<String>,
    pub child_labels: HashMap<char, Link>,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self {
            has_values: false,
            values: Vec::new(),
            child_labels: HashMap::new(),
        }
    }
}

impl PrefixTree {
    pub fn new() -> Self {
        let root_node = Node::new();
        let root_ptr = Box::into_raw(Box::new(root_node));
        Self { root: root_ptr }
    }

    pub unsafe fn insert(&mut self, key: String, value: String) {
        let mut cur_node_link = self.root;
        for label in key.chars() {
            match (*cur_node_link).go(&label) {
                None => {
                    cur_node_link = (*cur_node_link).add_label(&label);
                }
                Some(link) => {
                    cur_node_link = link;
                }
            }
        }
        (*cur_node_link).has_values = true;
        (*cur_node_link).values.push(value);
    }

    pub unsafe fn get(&self, key: String) -> Option<Vec<String>> {
        let mut cur_node_link = self.root;
        for label in key.chars() {
            match (*cur_node_link).go(&label) {
                None => {
                    return None;
                }
                Some(link) => {
                    cur_node_link = link;
                }
            }
        }
        Some((*cur_node_link).values.clone())
    }
}

impl Node {
    pub fn go(&self, label: &char) -> Option<Link> {
        self.child_labels.get(label).copied()
    }

    pub fn add_label(&mut self, label: &char) -> Link {
        let new_node_link: Link = Box::into_raw(Box::new(Node::new()));
        self.child_labels.insert(*label, new_node_link);
        new_node_link
    }
}
