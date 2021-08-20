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

    /// **O(n)**, heapify in place
    pub fn from(v: Vec<T>, op: F) -> Self {
        let mut h = Self { v, op };
        h.heapify();
        h
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
            self.v.swap(0, n - 1);
        }
        self.v.pop().map(|popped| {
            if n > 1 {
                self.down_heap(0);
            }
            popped
        })
    }

    /// **O(1)**, peek min item
    pub fn peek(&self) -> Option<&T> {
        self.v.get(0)
    }

    /// **O(n)**, heapify
    pub fn heapify(&mut self) {
        for pos in (0..(self.v.len() / 2)).rev() {
            self.down_heap(pos);
        }
    }

    /// **O(1)**, get bfs iterator, but full search will take **O(n)** step
    pub fn bfs(&self) -> impl Iterator<Item = &T> {
        self.v.iter()
    }

    /// **O(log(n))**, heapify subtree
    pub fn down_heap(&mut self, pos: usize) {
        let val = (self.op)(&self.v[pos]);
        if let Some(lc) = self.v.get(pos * 2 + 1) {
            if val > (self.op)(lc) {
                self.v.swap(pos, pos * 2 + 1);
                self.down_heap(pos * 2 + 1);
            }
        }
        let val = (self.op)(&self.v[pos]);
        if let Some(rc) = self.v.get(pos * 2 + 2) {
            if val > (self.op)(rc) {
                self.v.swap(pos, pos * 2 + 2);
                self.down_heap(pos * 2 + 2);
            }
        }
    }

    /// **O(log(n))**, heapify to root
    pub fn up_heap(&mut self, pos: usize) {
        if pos != 0 {
            let parent = (pos - 1) / 2;
            if (self.op)(&self.v[parent]) > (self.op)(&self.v[pos]) {
                self.v.swap(pos, parent);
                self.up_heap(parent);
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
    fn heapify_test() {
        let v = vec![1, 3, -5, -4, 2];
        let mut abs_mh = BHeapSet::from(v, |&x| x * x);
        assert_eq!(abs_mh.pop(), Some(1));
        assert_eq!(abs_mh.pop(), Some(2));
        assert_eq!(abs_mh.pop(), Some(3));
        assert_eq!(abs_mh.pop(), Some(-4));
        assert_eq!(abs_mh.pop(), Some(-5));
        assert_eq!(abs_mh.pop(), None);
    }

    #[test]
    fn bfs_test() {
        let mut max_heap = BHeapSet::new(|&x| std::cmp::Reverse(x));
        max_heap.push(10);
        max_heap.push(100);
        max_heap.push(1);
        assert_eq!(max_heap.bfs().cloned().collect::<Vec<_>>(), vec![100, 10, 1]);
        assert_eq!(max_heap.pop(), Some(100));
        assert_eq!(max_heap.pop(), Some(10));
        assert_eq!(max_heap.pop(), Some(1));
        assert_eq!(max_heap.pop(), None);
    }
}
