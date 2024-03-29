use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

struct Node<T> {
    item: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Drop for LinkedList<T> {
    /// **O(n)**, drop remain nodes
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> Extend<T> for LinkedList<T> {
    /// **O(n)**, extends items as tails
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.enqueue(item);
        }
    }
}

impl<T> LinkedList<T> {
    /// **O(1)**, return empty linked list
    pub fn new() -> Self {
        LinkedList { tail: None, head: None, len: 0 }
    }

    /// **O(1)**, return length of the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// **O(1)**, return true if the list has no item, else return false
    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    /// **O(1)**, peek the top of the list
    pub fn peek_head(&self) -> Option<Ref<T>> {
        if let Some(head_node) = &self.head {
            Some(Ref::map(head_node.borrow(), |node| &node.item))
        } else {
            None
        }
    }

    /// **O(1)**, peek the end of the list
    pub fn peek_tail(&self) -> Option<Ref<T>> {
        if let Some(tail_node) = &self.tail {
            Some(Ref::map(tail_node.borrow(), |node| &node.item))
        } else {
            None
        }
    }

    /// **O(1)**, add first node in the empty linked list
    fn add_first_node(&mut self, item: T) {
        let node = RefCell::new(Node { item, next: None });
        let node_rc = Rc::new(node);
        self.head = Some(node_rc.clone());
        self.tail = Some(node_rc);
    }

    /// **O(1)**, add new item to the top of the list
    pub fn push(&mut self, item: T) {
        self.len += 1;
        if let Some(head_node_rc) = self.head.take() {
            // the case that the linked list is not empty
            let new_head = RefCell::new(Node { item, next: Some(head_node_rc.clone()) });
            self.head = Some(Rc::new(new_head));
        } else {
            // the case that the linked list is empty
            self.add_first_node(item)
        }
    }

    /// **O(1)**, return and remove head item
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head_node_rc| {
            self.len -= 1;
            if let Some(next_node_rc) = head_node_rc.borrow_mut().next.take() {
                // the case that the linked list does not become empty
                self.head = Some(next_node_rc.clone());
            } else {
                // the case that the linked list become empty
                self.tail = None;
            }
            Rc::try_unwrap(head_node_rc).ok().unwrap().into_inner().item
        })
    }

    /// **O(1)**, add new item to the end of the list
    pub fn enqueue(&mut self, item: T) {
        self.len += 1;
        if let Some(tail_node_rc) = self.tail.take() {
            // the case that the linked list is not empty
            let new_tail = Rc::new(RefCell::new(Node { item, next: None }));
            tail_node_rc.borrow_mut().next = Some(new_tail.clone());
            self.tail = Some(new_tail);
        } else {
            // the case that the linked list is empty
            self.add_first_node(item);
        }
    }

    /// **O(1)**, return and remove head item
    pub fn dequeue(&mut self) -> Option<T> {
        self.pop()
    }

    /// **O(1)**, append the other linked list
    pub fn append(&mut self, other: &mut Self) {
        self.len += other.len();
        other.len = 0;
        if let Some(tail_node_rc) = self.tail.take() {
            tail_node_rc.borrow_mut().next = other.head.take();
            self.tail = other.tail.take();
        } else {
            self.head = other.head.take();
            self.tail = other.tail.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operation() {
        let mut stack = LinkedList::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert!(stack.peek_head().is_none());
        assert!(stack.peek_tail().is_none());
        stack.push(5);
        assert_eq!(stack.len(), 1);
        assert_eq!(*stack.peek_head().unwrap(), 5);
        assert_eq!(*stack.peek_tail().unwrap(), 5);
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.len(), 0);
        stack.push(2);
        stack.push(3);
        let tail1 = *stack.peek_tail().unwrap();
        stack.push(4);
        let tail2 = *stack.peek_tail().unwrap();
        assert_eq!(tail1, tail2);
        assert_eq!(*stack.peek_head().unwrap(), 4);
        assert_eq!(*stack.peek_tail().unwrap(), 2);
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), None);
        assert!(stack.peek_head().is_none());
        stack.push(10);
        assert_eq!(*stack.peek_head().unwrap(), 10);
        assert_eq!(*stack.peek_tail().unwrap(), 10);
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn test_queue_operation() {
        let mut queue = LinkedList::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
        assert!(queue.peek_tail().is_none());
        queue.enqueue("Rust");
        assert_eq!(queue.len(), 1);
        assert_eq!(*queue.peek_tail().unwrap(), "Rust");
        assert_eq!(queue.dequeue(), Some("Rust"));
        assert_eq!(queue.len(), 0);
        queue.enqueue("The");
        queue.enqueue("Programing");
        queue.enqueue("Language");
        queue.enqueue("Rust");
        assert_eq!(*queue.peek_tail().unwrap(), "Rust");
        assert_eq!(queue.dequeue(), Some("The"));
        assert_eq!(queue.dequeue(), Some("Programing"));
        assert_eq!(queue.dequeue(), Some("Language"));
        assert_eq!(queue.dequeue(), Some("Rust"));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.peek_tail().is_none());
        queue.enqueue("a");
        assert_eq!(*queue.peek_head().unwrap(), "a");
        assert_eq!(*queue.peek_tail().unwrap(), "a");
        assert_eq!(queue.dequeue(), Some("a"));
    }

    #[test]
    fn test_drop() {
        let raw: *mut _;
        {
            let mut ll = LinkedList::new();
            ll.push('r');
            ll.push('u');
            ll.push('s');
            ll.push('t');
            raw = &mut ll;
        }
        assert_eq!(unsafe { &mut *raw }.pop(), None);
    }

    #[test]
    fn test_extend() {
        let mut ll = LinkedList::new();
        ll.push('r');
        ll.extend(vec!['u', 's', 't']);
        // TODO implement iter and refactor test
        assert_eq!(ll.dequeue(), Some('r'));
        assert_eq!(ll.dequeue(), Some('u'));
        assert_eq!(ll.dequeue(), Some('s'));
        assert_eq!(ll.dequeue(), Some('t'));
        assert_eq!(ll.dequeue(), None);
    }

    #[test]
    fn test_append1() {
        let mut ll1 = LinkedList::new();
        ll1.enqueue(1);
        ll1.enqueue(2);
        let mut ll2 = LinkedList::new();
        ll2.enqueue(3);
        ll2.enqueue(4);
        assert_eq!(ll1.len(), 2);
        assert_eq!(ll2.len(), 2);
        ll1.append(&mut ll2);
        assert_eq!(ll2.dequeue(), None);
        assert_eq!(ll2.len(), 0);
        assert_eq!(*ll1.peek_tail().unwrap(), 4);
        assert_eq!(ll1.len(), 4);
        // TODO implement iter and refactor test
        assert_eq!(ll1.dequeue(), Some(1));
        assert_eq!(ll1.dequeue(), Some(2));
        assert_eq!(ll1.dequeue(), Some(3));
        assert_eq!(ll1.dequeue(), Some(4));
        assert_eq!(ll1.dequeue(), None);
    }

    #[test]
    fn test_append2() {
        {
            let mut ll1 = LinkedList::new();
            ll1.enqueue(1);
            ll1.enqueue(2);
            let mut ll2 = LinkedList::new();
            assert_eq!(ll1.len(), 2);
            assert_eq!(ll2.len(), 0);
            ll1.append(&mut ll2);
            assert_eq!(ll2.dequeue(), None);
            assert_eq!(ll1.len(), 2);
            assert_eq!(ll2.len(), 0);
            assert_eq!(ll1.dequeue(), Some(1));
            assert_eq!(ll1.dequeue(), Some(2));
            assert_eq!(ll1.dequeue(), None);
        }
        {
            let mut ll1 = LinkedList::new();
            let mut ll2 = LinkedList::new();
            ll2.enqueue(3);
            ll2.enqueue(4);
            assert_eq!(ll1.len(), 0);
            assert_eq!(ll2.len(), 2);
            ll1.append(&mut ll2);
            assert_eq!(ll2.dequeue(), None);
            assert_eq!(ll1.len(), 2);
            assert_eq!(ll2.len(), 0);
            assert_eq!(ll1.dequeue(), Some(3));
            assert_eq!(ll1.dequeue(), Some(4));
            assert_eq!(ll1.dequeue(), None);
        }
        {
            let mut ll1: LinkedList<()> = LinkedList::new();
            let mut ll2 = LinkedList::new();
            assert_eq!(ll1.len(), 0);
            assert_eq!(ll2.len(), 0);
            ll1.append(&mut ll2);
            assert_eq!(ll2.dequeue(), None);
            assert_eq!(ll1.len(), 0);
            assert_eq!(ll2.len(), 0);
            assert_eq!(ll1.dequeue(), None);
        }
    }
}
