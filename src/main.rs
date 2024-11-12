use std::{collections::HashMap, vec};

pub struct Node {
    pub count: i32,
    pub child: HashMap<String, Box<Node>>,
}

pub struct LookUpTree {
    pub root_node: Box<Node>,
}

impl LookUpTree {
    pub fn new() -> LookUpTree {
        LookUpTree {
            root_node: Box::new(Node {
                count: 0,
                child: HashMap::new(),
            }),
        }
    }

    pub fn insert(&mut self, keychain: &Vec<&str>, count: i32) {
        let mut parent = &mut self.root_node;
        parent.count += count;
        for key in keychain {
            // if child is present, fetch the node
            if parent.child.contains_key(key.to_owned()) {
                parent = parent.child.get_mut(key.to_owned()).unwrap();
                parent.count += count;
            }
            // if child is not present, create a new node
            else {
                parent.child.insert(
                    key.to_string(),
                    Box::new(Node {
                        count,
                        child: HashMap::new(),
                    }),
                );
                parent = parent.child.get_mut(key.to_owned()).unwrap();
            }
        }
    }
}

impl Default for LookUpTree {
    fn default() -> Self {
        Self::new()
    }
}

pub fn search(cur_node: &Node, key: &Vec<&str>, postfix_key_count: &[u8], index: usize) -> i32 {
    if index == key.len() {
        return cur_node.count;
    };

    let current_key = key[index];
    
    // if the child is a wildcard
    if current_key == "*" {
        // if the whole postfix is a wildcard
        if postfix_key_count[index] == 0 {
            return cur_node.count;
        } else {
            let mut sum = 0;
            for (_, child) in cur_node.child.iter() {
                sum += search(child, key, postfix_key_count, index + 1);
            }
            return sum;
        }
    }
    // if the child is present
    else if cur_node.child.contains_key(current_key) {
        let current_child = cur_node.child.get(current_key).unwrap();

        return search(current_child, key, postfix_key_count, index + 1);
    }

    // if the child is not present
    0
}

pub fn main() {
    let mut tree = LookUpTree::new();
    tree.insert(&vec!["a", "b", "c", "d"], 1);
    tree.insert(&vec!["a", "c", "b", "d"], 2);
    tree.insert(&vec!["b", "b", "c", "d"], 3);
    tree.insert(&vec!["b", "b", "c", "a"], 4);
    tree.insert(&vec!["b", "c", "c", "c"], 5);
    tree.insert(&vec!["c", "b", "d", "a"], 6);
    tree.insert(&vec!["c", "a", "b", "c"], 7);

    let query_strings = [
        "a,b,c,d", "*,*,*,*", "*,b,*,*", "*,b,c,*", "*,b,c,d", "a,b,c,*", "a,b,*,*",
    ];

    // each query is a vector of strings For eg. vec!["a", "b", "c", "d"],
    let queries: Vec<Vec<&str>> = query_strings
        .iter()
        .map(|x| x.split(",").collect())
        .collect();

    for query in queries {
        // count the number of keys in the postfix order
        let mut postfix_key_count = vec![];
        let mut key_count = 0;

        for key in query.iter().cloned().rev() {
            if key != "*" {
                key_count += 1;
            }
            postfix_key_count.push(key_count);
        }

        // reverse the order from prefix to postfix
        postfix_key_count.reverse();

        let result = search(&tree.root_node, &query, &postfix_key_count, 0);
        println!("Result for {:?} is {}", query, result);
    }
}
