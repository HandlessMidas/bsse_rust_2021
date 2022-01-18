use crate::pref_tree::PrefixTree;

pub struct ParsingTree {
    pub root: Link,
}

type Link = *mut Node;

pub struct Node {
    pub values: Vec<String>,
    pub children: Vec<Link>,
    pub is_empty: bool,
    pub is_leaf: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            is_empty: false,
            is_leaf: false,
            values: vec![],
            children: vec![],
        }
    }

    fn add_child(&mut self, values: &[String]) -> Link {
        let mut new_node = Self::new();
        new_node.values = values.to_owned();
        let new_link = Box::into_raw(Box::new(new_node));
        self.children.push(new_link);
        new_link
    }

    fn comb(&self) -> Vec<String> {
        if self.is_leaf {
            return vec![String::from("")];
        }

        if self.is_empty {
            return vec![];
        }

        let mut result = vec![];

        let suffs: Vec<String> = self
            .children
            .iter()
            .flat_map(|x| unsafe { (**x).comb() })
            .collect();

        for pref in self.values.iter() {
            for suff in suffs.iter() {
                if pref.is_empty() {
                    result.push(suff.clone());
                    continue;
                }
                if suff.is_empty() {
                    result.push(pref.clone());
                    continue;
                }
                let mut new_value = String::new();
                new_value.push_str(pref);
                new_value.push(' ');
                new_value.push_str(suff);
                result.push(new_value);
            }
        }

        result
    }
}

impl ParsingTree {
    fn new() -> Self {
        let mut root = Node::new();
        root.values.push(String::from(""));
        Self {
            root: Box::into_raw(Box::new(root)),
        }
    }
}

pub struct Parser {
    parsing_tree: ParsingTree,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            parsing_tree: ParsingTree::new(),
        }
    }

    unsafe fn partial_parse(
        &mut self,
        link: Link,
        input: &str,
        prefix_tree: &PrefixTree,
        prev_is_num: bool,
    ) -> bool {
        if input.is_empty() {
            let mut new_node_link = (*link).add_child(&Vec::new());
            (*new_node_link).is_leaf = true;
            return true;
        }
        let mut cur_link = prefix_tree.root;
        let mut success = false;
        let mut try_num = true;
        for (index, label) in input.chars().enumerate() {
            match (*cur_link).go(&label) {
                None => {
                    break;
                }
                Some(link) => cur_link = link,
            }
            if (*cur_link).has_values {
                try_num = false;
                let new_node_link = (*link).add_child(&(*cur_link).values);
                let child_success = self.partial_parse(
                    new_node_link,
                    &input[index + 1..],
                    prefix_tree,
                    false,
                );
                if child_success {
                    success = true
                }
            }
        }
        if try_num && !prev_is_num {
            let values = vec![input.chars().next().unwrap().to_string()];
            let new_node_link = (*link).add_child(&values);
            let child_success =
                self.partial_parse(new_node_link, &input[1..], prefix_tree, true);
            if child_success {
                success = true
            }
        }
        (*link).is_empty = !success;
        success
    }

    pub unsafe fn parse(&mut self, input: &str, prefix_tree: &PrefixTree) -> Vec<String> {
        self.partial_parse(self.parsing_tree.root, input, prefix_tree, false);
        (*self.parsing_tree.root).comb()
    }
}
