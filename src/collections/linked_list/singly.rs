pub struct LinkedList<T> {
    head: Option<Node<T>>,
    tail: *mut Node<T>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// **O(1)**, return empty linked list
    pub fn new() -> Self {
        LinkedList {
            tail: std::ptr::null_mut(),
            head: None,
            len: 0,
        }
    }

    /// **O(1)**, return length of the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// **O(1)**, return true if the list has no element, else return false
    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    /// **O(1)**, peek the top of the list
    pub fn peek_head(&self) -> Option<&T> {
        if let Some(head_node) = &self.head {
            Some(&head_node.element)
        } else {
            None
        }
    }

    /// **O(1)**, peek the end of the list
    pub fn peek_tail(&self) -> Option<&T> {
        if self.tail.is_null() {
            None
        } else {
            Some(&unsafe { &*self.tail }.element)
        }
    }

    /// **O(1)**, add first node in the empty linked list
    fn add_first_node(&mut self, element: T) {
        let mut node = Node {
            element,
            next: None,
        };
        self.tail = &mut node;
        self.head = Some(node);
    }

    /// **O(1)**, add new element to the top of the list
    pub fn push(&mut self, element: T) {
        self.len += 1;
        if let Some(head_node) = self.head.take() {
            // the case that the linked list is not empty
            let new_head = Node {
                element,
                next: Some(Box::new(head_node)),
            };
            self.head = Some(new_head);
        } else {
            // the case that the linked list is empty
            self.add_first_node(element)
        }
    }

    /// **O(1)**, return and remove head element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head_node| {
            self.len -= 1;
            if let Some(next_node) = head_node.next {
                // the case that the linked list does not become empty
                self.head = Some(*next_node);
            } else {
                // the case that the linked list become empty
                self.tail = std::ptr::null_mut();
            }
            head_node.element
        })
    }

    /// **O(1)**, add new element to the end of the list
    pub fn enqueue(&mut self, element: T) {
        self.len += 1;
        if self.tail.is_null() {
            // the case that the linked list is empty
            self.add_first_node(element);
        } else {
            let mut new_tail = Box::new(Node {
                element,
                next: None,
            });
            let old_tail = unsafe { &mut *self.tail };
            self.tail = &mut *new_tail;
            old_tail.next = Some(new_tail);
        }
    }

    /// **O(1)**, return and remove head element
    pub fn dequeue(&mut self) -> Option<T> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_operation_test() {
        let mut stack = LinkedList::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek_head(), None);
        assert_eq!(stack.peek_tail(), None);
        stack.push(5);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek_head(), Some(&5));
        assert_eq!(stack.peek_tail(), Some(&5));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.len(), 0);
        stack.push(2);
        stack.push(3);
        let tail1 = *stack.peek_tail().unwrap();
        stack.push(4);
        let tail2 = *stack.peek_tail().unwrap();
        assert_eq!(tail1, tail2);
        assert_eq!(stack.peek_head(), Some(&4));
        assert_eq!(stack.peek_tail(), Some(&2));
        // assert_eq!(stack.pop(), Some(4));
        // assert_eq!(stack.pop(), Some(3));
        // assert_eq!(stack.pop(), Some(2));
        // assert_eq!(stack.pop(), None);
        // assert_eq!(stack.peek_head(), None);
        // stack.push(10);
        // assert_eq!(stack.peek_head(), Some(&10));
        // assert_eq!(stack.peek_tail(), Some(&10));
        // assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn queue_operation_test() {
        let mut queue = LinkedList::new();
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek_tail(), None);
        queue.enqueue("Rust");
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek_tail(), Some(&"Rust"));
        assert_eq!(queue.dequeue(), Some("Rust"));
        assert_eq!(queue.len(), 0);
        queue.enqueue("The");
        queue.enqueue("Programing");
        // queue.enqueue("Language");
        queue.enqueue("Rust");
        assert_eq!(queue.peek_tail(), Some(&"Rust"));
        assert_eq!(queue.dequeue(), Some("The"));
        assert_eq!(queue.dequeue(), Some("Programing"));
        assert_eq!(queue.dequeue(), Some("Language"));
        assert_eq!(queue.dequeue(), Some("Rust"));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek_tail(), None);
        queue.enqueue("a");
        assert_eq!(queue.peek_head(), Some(&"a"));
        assert_eq!(queue.peek_tail(), Some(&"a"));
        assert_eq!(queue.dequeue(), Some("a"));
    }

    #[test]
    fn standard_operation_test() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.len(), 0);
        assert_eq!(ll.peek_head(), None);
        assert_eq!(ll.peek_tail(), None);
        ll.push(5);
        assert_eq!(ll.len(), 1);
        assert_eq!(ll.peek_head(), Some(&5));
        assert_eq!(ll.peek_tail(), Some(&5));
        ll.enqueue(10);
        assert_eq!(ll.len(), 2);
        assert_eq!(ll.peek_head(), Some(&5));
        assert_eq!(ll.peek_tail(), Some(&10));
        let x = ll.pop();
        assert_eq!(ll.len(), 1);
        assert_eq!(x, Some(5));
        assert_eq!(ll.peek_head(), Some(&10));
        assert_eq!(ll.peek_tail(), Some(&10));
        let x = ll.dequeue();
        assert_eq!(ll.len(), 0);
        assert_eq!(x, Some(10));
        assert_eq!(ll.peek_head(), None);
        assert_eq!(ll.peek_tail(), None);
        let x = ll.pop();
        assert_eq!(ll.len(), 0);
        assert_eq!(x, None);
        assert_eq!(ll.peek_head(), None);
        assert_eq!(ll.peek_tail(), None);
    }
}
