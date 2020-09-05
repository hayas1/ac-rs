#![allow(dead_code)]

use num::Num;

struct SumSegmentTree<T: Num + Copy> {
    n: usize,
    binary_tree: Vec<T>,
}
impl<T: Num + Copy> SumSegmentTree<T> {
    /// O(n) # create segment tree (its monoid function is add, so we can calculate summation)
    fn new(data: &[T]) -> Self {
        let n = data.len();
        let mut binary_tree = vec![T::zero(); 2 * n.next_power_of_two() - 1];
        for (i, &data) in data.iter().enumerate() {
            binary_tree[n.next_power_of_two() - 1 + i] = data;
        }
        for i in (0..n.next_power_of_two() - 1).rev() {
            binary_tree[i] = binary_tree[2 * i + 1] + binary_tree[2 * i + 2];
        }
        SumSegmentTree { n, binary_tree }
    }

    /// O(log(n)) # update segment tree, leaf[k] = x
    fn update(&mut self, k: usize, x: T) {
        let i = self.n.next_power_of_two() - 1 + k;
        self.binary_tree[i] = x;
        if i > 0 {
            self.recursive_update((i - 1) / 2);
        }
    }

    /// O(log(i)) # update from leaf to root
    fn recursive_update(&mut self, i: usize) {
        self.binary_tree[i] = self.binary_tree[2 * i + 1] + self.binary_tree[2 * i + 2];
        if i > 0 {
            self.recursive_update((i - 1) / 2);
        }
    }

    /// O(log(n)) # calculate half-open interval summation from l to r (leaf index)
    fn query(&self, l: usize, r: usize) -> T {
        self.recursive_query(l, r, 0, 0, self.n.next_power_of_two())
    }

    /// O(log(n)-log(node)) # calculate summation from root to leaf
    fn recursive_query(&self, l: usize, r: usize, node: usize, from: usize, to: usize) -> T {
        if r <= from || to <= l {
            T::zero()
        } else if l <= from && to <= r {
            self.binary_tree[node]
        } else {
            let lv = self.recursive_query(l, r, node * 2 + 1, from, (from + to) / 2);
            let rv = self.recursive_query(l, r, node * 2 + 2, (from + to) / 2, to);
            lv + rv
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_test() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = SumSegmentTree::new(&data);
        assert_eq!(
            t.binary_tree,
            vec![40, 27, 13, 12, 15, 13, 0, 10, 2, 3, 12, 13, 0, 0, 0]
        );
        t.update(3, 22);
        assert_eq!(
            t.binary_tree,
            vec![50, 37, 13, 12, 25, 13, 0, 10, 2, 3, 22, 13, 0, 0, 0]
        );
    }

    #[test]
    fn query_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7];
        let t = SumSegmentTree::new(&data);
        assert_eq!(t.query(2, 6), 14);
        assert_eq!(t.query(2, 5), 9);
        assert_eq!(t.query(3, 5), 7);
        assert_eq!(t.query(3, 6), 12);
    }

    #[test]
    fn query_bound_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7];
        let t = SumSegmentTree::new(&data);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
        assert_eq!(t.query(9, 3), 0);
    }

    #[test]
    fn update_query_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut t = SumSegmentTree::new(&data);
        assert_eq!(t.query(3, 5), 7);
        assert_eq!(t.query(2, 7), 20);
        assert_eq!(t.query(0, 11), 55);
        t.update(5, 10);
        assert_eq!(t.query(3, 5), 7);
        assert_eq!(t.query(2, 7), 25);
        assert_eq!(t.query(0, 11), 60);
    }

    #[test]
    fn minimum1_tree_test() {
        let data = vec![4];
        let mut t = SumSegmentTree::new(&data);
        assert_eq!(t.query(0, 2), 4);
        assert_eq!(t.query(0, 1), 4);
        assert_eq!(t.query(0, 0), 0);
        t.update(0, 100);
        assert_eq!(t.query(0, 2), 100);
        assert_eq!(t.query(0, 1), 100);
        assert_eq!(t.query(0, 0), 0);
    }

    #[test]
    fn minimum0_tree_test() {
        let data = Vec::<usize>::new();
        let t = SumSegmentTree::new(&data);
        assert_eq!(t.query(0, 2), 0);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
    }
}
