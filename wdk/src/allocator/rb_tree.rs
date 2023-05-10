use crate::allocator::rc::Rc;
use core::sync::atomic::Ordering;
use wdk_sys::base::POOL_TYPE;

enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Rc<Node<K, V>>>,
    right: Option<Rc<Node<K, V>>>,
    color: Color,
}

pub struct RedBlackTree<K, V> {
    root: Option<Rc<Node<K, V>>>,
    pool_type: POOL_TYPE,
    tag: u32,
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn new(pool_type: POOL_TYPE, tag: u32) -> Self {
        RedBlackTree {
            root: None,
            pool_type,
            tag,
        }
    }

    fn is_red(self, node: &Option<Rc<Node<K, V>>>) -> bool {
        match node {
            Some(n) => n.color == Color::Red,
            None => false,
        }
    }
    //
    // fn rotate_left(self, node: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    //     let mut right = node.right.unwrap();
    //     node.right = right.left;
    //     right.left = Some(node);
    //     right.color = node.color;
    //     node.color = Color::Red;
    //     right
    // }
    //
    // fn rotate_right(self, node: Rc<Node<K, V>>) -> Rc<Node<K, V>> {
    //     let mut left = node.left.unwrap();
    //     node.left = left.right;
    //     left.right = Some(node);
    //     left.color = node.color;
    //     node.color = Color::Red;
    //     left
    // }
    //
    // fn flip_colors(self, node: &mut Node<K, V>) {
    //     node.color = Color::Red;
    //     node.left.as_mut().unwrap().color = Color::Black;
    //     node.right.as_mut().unwrap().color = Color::Black;
    // }
    //
    // fn insert_node(self, node: Option<Rc<Node<K, V>>>, key: K, value: V) -> Option<Rc<Node<K, V>>> {
    //     match node {
    //         Some(mut n) => {
    //             if key < n.key {
    //                 n.left = Self::insert_node(n.left, key, value);
    //             } else if key > n.key {
    //                 n.right = Self::insert_node(n.right, key, value);
    //             } else {
    //                 n.value = value;
    //             }
    //
    //             if Self::is_red(&n.right) && !Self::is_red(&n.left) {
    //                 n = Self::rotate_left(n);
    //             }
    //             if Self::is_red(&n.left) && Self::is_red(&n.left.as_ref().unwrap().left) {
    //                 n = Self::rotate_right(n);
    //             }
    //             if Self::is_red(&n.left) && Self::is_red(&n.right) {
    //                 Self::flip_colors(&mut n);
    //             }
    //
    //             Some(n)
    //         }
    //         None => Some(Rc::new(
    //             Node {
    //                 key,
    //                 value,
    //                 left: None,
    //                 right: None,
    //                 color: Color::Red,
    //             },
    //             self.pool_type,
    //             self.tag,
    //         )?),
    //     }
    // }
    //
    // pub fn insert(&mut self, key: K, value: V) {
    //     self.root = self.insert_node(self.root.take(), key, value);
    //     self.root.as_mut().unwrap().color = Color::Black;
    // }
    //
    // fn get_node(node: &Option<Rc<Node<K, V>>>, key: &K) -> Option<&V> {
    //     match node {
    //         Some(n) => {
    //             if key < &n.key {
    //                 Self::get_node(&n.left, key)
    //             } else if key > &n.key {
    //                 Self::get_node(&n.right, key)
    //             } else {
    //                 Some(&n.value)
    //             }
    //         }
    //         None => None,
    //     }
    // }
    //
    // pub fn get(&self, key: &K) -> Option<&V> {
    //     Self::get_node(&self.root, key)
    // }
}
