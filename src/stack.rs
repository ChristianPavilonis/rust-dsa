use std::{borrow::Borrow, cell::RefCell, ptr::NonNull, rc::Rc};

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Self {
            value: item,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct Stack<T> {
    pub len: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, item: T) {
        self.len += 1;

        let mut node = Box::new(Node::new(item));

        if let Some(tail_ptr) = self.tail {
            node.prev = Some(tail_ptr);
        }

        let node_ptr = NonNull::new(Box::into_raw(node));

        if self.head.is_none() {
            self.head = node_ptr;
        }

        self.tail = node_ptr;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());

            self.tail = old_tail.prev;
            self.len = self.len.checked_sub(1).unwrap_or(0);

            old_tail.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.tail
            .map(|tail_ptr| unsafe { &(*tail_ptr.as_ptr()).value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len, 0);
    }
}
