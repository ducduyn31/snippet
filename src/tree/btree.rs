use std::fmt::Debug;

const MAX_DEGREE: usize = 2;

#[derive(Debug)]
struct Node<K: Ord + Debug + Clone, V: Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<Node<K, V>>>,
    leaf: bool,
}

impl<K: Ord + Debug + Clone, V: Clone> Node<K, V> {
    fn new(leaf: bool) -> Self {
        Node {
            leaf,
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct BTree<K: Ord + Debug + Clone, V: Clone> {
    root: Option<Box<Node<K, V>>>,
    degree: usize,
}

impl<K: Ord + Debug + Clone, V: Clone> Default for BTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord + Debug + Clone, V: Clone> BTree<K, V> {
    pub fn new() -> Self {
        BTree {
            root: None,
            degree: MAX_DEGREE,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match &mut self.root {
            None => {
                let mut root = Box::new(Node::new(true));
                root.keys.push(key);
                root.values.push(value);
                self.root = Some(root);
            }
            Some(_) => {
                let mut root = self.root.take().unwrap();
                if root.keys.len() == 2 * self.degree - 1 {
                    let mut new_root = Box::new(Node::new(false));
                    new_root.children.push(root);
                    let mut temp_root = Some(new_root);

                    if let Some(root) = temp_root.as_mut() {
                        self.split_child(root, 0);
                        self.insert_non_full(root, key, value);
                    }
                    self.root = temp_root;
                } else {
                    self.insert_non_full(&mut root, key, value);
                    self.root =  Some(root);
                }
            }
        }
    }

    fn insert_non_full(&mut self, node: &mut Box<Node<K, V>>, key: K, value: V) {
        let mut i = node.keys.len();

        if node.leaf {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            node.keys.insert(i, key);
            node.values.insert(i, value);
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            if node.children[i].keys.len() == 2 * self.degree - 1 {
                self.split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_non_full(&mut node.children[i], key, value);
        }
    }

    fn split_child(&mut self, parent: &mut Box<Node<K, V>>, index: usize) {
        let child = &mut parent.children[index];
        let mut new_node = Box::new(Node::new(child.leaf));

        let mid_index = self.degree - 1;
        let mid_key = child.keys.remove(mid_index);
        let mid_value = child.values.remove(mid_index);

        new_node.keys = child.keys.split_off(self.degree - 1);
        new_node.values = child.values.split_off(self.degree - 1);
        if !child.leaf {
            new_node.children = child.children.split_off(self.degree);
        }

        parent.keys.insert(index, mid_key);
        parent.values.insert(index, mid_value);
        parent.children.insert(index + 1, new_node);
    }

    pub fn search(&self, key: K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref root) => self.search_node(root, key),
        }
    }

    fn search_node<'a>(&self, node: &'a Node<K, V>, key: K) -> Option<&'a V> {
        let mut i = 0;
        while i < node.keys.len() && key > node.keys[i] {
            i += 1;
        }
        if i < node.keys.len() && key == node.keys[i] {
            Some(&node.values[i])
        } else if node.leaf {
            None
        } else {
            self.search_node(&node.children[i], key)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}


