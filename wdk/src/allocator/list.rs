use super::rc::Rc;
use crate::error::Error;
use wdk_sys::base::POOL_TYPE;

pub struct NodeEntry<T> {
    pub value: T,
    prev: Option<Rc<NodeEntry<T>>>,
    next: Option<Rc<NodeEntry<T>>>,
}

pub struct List<T> {
    pool_type: POOL_TYPE,
    tag: u32,
    head: Rc<NodeEntry<T>>,
}

impl<T> List<T> {
    pub fn new(data: T, pool_type: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        Ok(List {
            pool_type,
            tag,

            head: Rc::new(
                NodeEntry {
                    value: data,
                    prev: None,
                    next: None,
                },
                pool_type,
                tag,
            )?,
        })
    }

    pub fn insert_head(&mut self, node: NodeEntry<T>) -> Result<(), Error> {
        let mut new_node = Rc::new(node, self.pool_type, self.tag)?;
        if let Some(mut next) = self.head.next.take() {
            next.prev = Some(new_node.clone());
            new_node.next = Some(next);
        }
        self.head.next = Some(new_node.clone());
        new_node.prev = Some(self.head.clone());
        Ok(())
    }

    pub fn insert_tail(&mut self, node: NodeEntry<T>) -> Result<(), Error> {
        let mut new_node = Rc::new(node, self.pool_type, self.tag)?;
        if let Some(mut prev) = self.head.prev.take() {
            prev.next = Some(new_node.clone());
            new_node.prev = Some(prev);
        }
        self.head.prev = Some(new_node.clone());
        new_node.next = Some(self.head.clone());
        Ok(())
    }

    pub fn remove_tail(&mut self) -> Option<Rc<NodeEntry<T>>> {
        if let Some(mut tail) = self.head.prev.take() {
            if let Some(prev) = tail.prev.take() {
                self.head.prev = Some(prev);
            }
            tail.next.take();
            Some(tail)
        } else {
            None
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.clone();
        while let Some(mut next) = head.next.take() {
            if let Some(next2) = next.next.take() {
                head.prev.take();
                head = next2;
            }
        }
    }
}
