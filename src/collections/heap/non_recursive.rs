pub struct BHeapSet<T, U, F>
where
    U: Ord,
    F: Fn(&T) -> U,
{
    v: Vec<T>,
    op: F,
}

impl<T, U, F> BHeapSet<T, U, F>
where
    U: Ord,
    F: Fn(&T) -> U,
{
    /// **O(1)**, create min heap with operation that return total order type
    pub fn new(op: F) -> Self {
        Self { v: Vec::new(), op }
    }

    /// **O(1)**, return the number of elements in this heap
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// **O(log(n))**, push new item
    pub fn push(&mut self, item: T) {
        let n = self.len();
        self.v.push(item);
        self.up_heap(n);
    }

    /// **O(log(n))**, pop min item
    pub fn pop(&mut self) -> Option<T> {
        let n = self.len();
        if n > 0 {
            let popped = self.v.swap_remove(0);
            self.down_heap(0);
            Some(popped)
        } else {
            None
        }
    }

    /// **O(log(n))**, heapify subtree
    pub fn down_heap(&mut self, pos: usize) {
        let mut current = pos;
        while current < self.len() / 2 {
            let mut swap_child = 2 * current + 1; // left child is exist, because of while condition
            if swap_child + 1 < self.len()
                && (self.op)(&self.v[swap_child]) > (self.op)(&self.v[swap_child + 1])
            {
                swap_child += 1;
            }
            if (self.op)(&self.v[current]) > (self.op)(&self.v[swap_child]) {
                self.v.swap(current, swap_child);
                current = swap_child;
            } else {
                break;
            }
        }
    }

    /// **O(log(n))**, heapify to root
    pub fn up_heap(&mut self, pos: usize) {
        let mut current = pos;
        while 0 < current && current < self.len() {
            let parent = (current - 1) / 2;
            if (self.op)(&self.v[parent]) > (self.op)(&self.v[current]) {
                self.v.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_heap_test1() {
        let mut min_heap = BHeapSet::new(|&x| x);
        assert_eq!(min_heap.pop(), None);
        min_heap.push(100);
        min_heap.push(19);
        min_heap.push(123);
        assert_eq!(min_heap.pop(), Some(19));
        assert_eq!(min_heap.pop(), Some(100));
        assert_eq!(min_heap.pop(), Some(123));
        assert_eq!(min_heap.pop(), None);
    }

    #[test]
    fn min_heap_test2() {
        let mut min_heap = BHeapSet::new(|&x| x);
        assert_eq!(min_heap.pop(), None);
        min_heap.push(100);
        min_heap.push(19);
        min_heap.push(123);
        min_heap.push(1);
        min_heap.push(150);
        min_heap.push(13);
        min_heap.push(13);
        min_heap.push(19);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(13));
        assert_eq!(min_heap.pop(), Some(13));
        assert_eq!(min_heap.pop(), Some(19));
        assert_eq!(min_heap.pop(), Some(19));
        assert_eq!(min_heap.pop(), Some(100));
        assert_eq!(min_heap.pop(), Some(123));
        assert_eq!(min_heap.pop(), Some(150));
        assert_eq!(min_heap.pop(), None);
    }
}
