use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}
impl<T> Node<T>
{
    pub fn new(item: T) -> Self {
        Self {
            value: item,
            next: None,
        }
    }
}

#[derive(Debug)]
struct Queue<T> {
    pub len: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Queue<T>
{
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.len += 1;

        let mut node = Box::new(Node::new(item));
        node.next = None;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
    }

    pub fn deque(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());

            self.head = old_head.next;
            self.len = self.len.checked_add_signed(-1).unwrap_or(0);
            old_head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.map(|head_ptr| unsafe {
            &(*head_ptr.as_ptr()).value
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.deque(), Some(1));
        assert_eq!(queue.deque(), Some(2));
        assert_eq!(queue.deque(), Some(3));
        assert_eq!(queue.deque(), None);
    }
}
