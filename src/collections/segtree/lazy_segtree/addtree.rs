use num::{Integer, NumCast};

pub struct AddTree<T> {
    n: usize,
    binary_tree: Vec<(T, T)>, // 1-indexed binary tree (parent: x/2, left_child: x*2, right_child: x*2+1, sibling: x^1, is_x_left_child: x%2==0, is_x_right_child: x%2==1)
    e: T,
    f: fn(T, T) -> T,
}
impl<T: Integer + NumCast + Copy> AddTree<T> {
    /// **O(n)**, create segment tree. (e is identity element for a function f in type T)
    pub fn new(data: &[T]) -> Self {
        let (e, f) = (T::zero(), |a, b| a + b);
        let (n, binary_tree) =
            (data.len(), vec![(e, T::zero()); 2 * data.len().next_power_of_two()]);
        let segment_tree = AddTree { n, binary_tree, e, f };
        segment_tree.init(data)
    }

    /// **O(n)**, init segment tree by given data.
    fn init(mut self, data: &[T]) -> Self {
        let leaf_offset = self.leaf_offset();
        for (i, &di) in data.iter().enumerate() {
            self.binary_tree[leaf_offset + i] = (di, T::zero());
        }
        for i in (1..leaf_offset).rev() {
            self.binary_tree[i] =
                ((self.f)(self.binary_tree[i * 2].0, self.binary_tree[i * 2 + 1].0), T::zero());
        }
        self
    }

    /// **O(1)**, get beginning index of the segment tree leaf.
    pub fn leaf_offset(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// **O(1)**, get size of leaves
    pub fn num_of_leaf(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// **O(log(n))**, update the half interval [l, r) with add x.
    pub fn update_range(&mut self, l: usize, r: usize, x: T) {
        self.recursive_update_range(l, r, 1, 0, self.num_of_leaf(), x)
    }

    /// **O(log^2(n))?**, set lazy value from root to leaf
    pub fn recursive_update_range(
        &mut self,
        l: usize,
        r: usize,
        node: usize,
        from: usize,
        to: usize,
        x: T,
    ) {
        self.propagation(node);
        if r <= from || to <= l {
            return;
        } else if l <= from && to <= r {
            let (value, lazy) = self.binary_tree[node];
            self.binary_tree[node] = (value, lazy + x * T::from(to - from).unwrap());
            self.propagation(node);
        } else {
            let mid = (from + to) / 2;
            self.recursive_update_range(l, r, node * 2, from, mid, x);
            self.recursive_update_range(l, r, node * 2 + 1, mid, to, x);
            self.binary_tree[node] = (
                (self.f)(self.binary_tree[node * 2].0, self.binary_tree[node * 2 + 1].0),
                T::zero(),
            );
        }
    }

    /// **O(log^2(n))?**, calculate half-open interval summation from l to r(leaf index). and update lazy
    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.recursive_query(l, r, 1, 0, self.num_of_leaf())
    }

    /// **O(log^2(n))?**, calculate summation from root to leaf.
    pub fn recursive_query(
        &mut self,
        l: usize,
        r: usize,
        node: usize,
        from: usize,
        to: usize,
    ) -> T {
        self.propagation(node);
        if r <= from || to <= l {
            self.e
        } else if l <= from && to <= r {
            self.binary_tree[node].0
        } else {
            let mid = (from + to) / 2;
            (self.f)(
                self.recursive_query(l, r, node * 2, from, mid),
                self.recursive_query(l, r, node * 2 + 1, mid, to),
            )
        }
    }

    /// **O(log(n))?**, propagate lazy value to children
    pub fn propagation(&mut self, node: usize) {
        let (value, lazy) = self.binary_tree[node];
        self.binary_tree[node] = (value + lazy, lazy - lazy);
        if node < self.leaf_offset() {
            // propagation to children
            let (_value, lazy) = self.binary_tree[node];
            let (l_value, l_lazy) = self.binary_tree[node * 2];
            self.binary_tree[node * 2] = (l_value, l_lazy + lazy / T::from(2).unwrap());
            let (r_value, r_lazy) = self.binary_tree[node * 2 + 1];
            self.binary_tree[node * 2 + 1] = (r_value, r_lazy + lazy / T::from(2).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_query1() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = AddTree::new(&data);
        assert_eq!(
            t.binary_tree.iter().map(|&(v, _l)| v).collect::<Vec<_>>(),
            vec![0, 40, 27, 13, 12, 15, 13, 0, 10, 2, 3, 12, 13, 0, 0, 0]
        );
        t.update_range(3, 4, 10);
        assert_eq!(
            t.binary_tree.iter().map(|&(v, _l)| v).collect::<Vec<_>>(),
            vec![0, 50, 37, 13, 12, 25, 13, 0, 10, 2, 3, 22, 13, 0, 0, 0]
        );
        assert_eq!(
            t.binary_tree.iter().map(|&(_v, l)| l).collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(t.query(3, 4), 22);
        assert_eq!(
            t.binary_tree.iter().map(|&(v, _l)| v).collect::<Vec<_>>(),
            vec![0, 50, 37, 13, 12, 25, 13, 0, 10, 2, 3, 22, 13, 0, 0, 0]
        );
        assert_eq!(t.query(1, 4), 27);
        assert_eq!(
            t.binary_tree.iter().map(|&(v, _l)| v).collect::<Vec<_>>(),
            vec![0, 50, 37, 13, 12, 25, 13, 0, 10, 2, 3, 22, 13, 0, 0, 0]
        );
    }

    #[test]
    fn test_update_query2() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = AddTree::new(&data);
        assert_eq!(t.query(1, 3), 5);
        t.update_range(1, 3, 10);
        assert_eq!(t.query(1, 3), 25);
        assert_eq!(t.query(1, 4), 37);
    }

    #[test]
    fn test_update_query3() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = AddTree::new(&data);
        assert_eq!(t.query(0, 2), 12);
        t.update_range(1, 3, 10);
        assert_eq!(t.query(0, 2), 22);
        assert_eq!(t.query(1, 3), 25);
    }
}
