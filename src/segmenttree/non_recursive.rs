#![allow(dead_code)]

struct SegmentTree<T: Copy> {
    n: usize,            // size of data
    f: fn(T, T) -> T,    // binary operation of the monoid
    e: T,                // identity element of the monoid
    binary_tree: Vec<T>, // 1-indexed binary tree (parent: x/2, left_child: x*2, right_child: x*2+1, sibling: x^1, is_x_left_child: x%2==0, is_x_right_child: x%2==1)
}
impl<T: Copy> SegmentTree<T> {
    /// O(n) # create segment tree (e is identity element for a function f in type T)
    fn new(data: &[T], e: T, f: fn(T, T) -> T) -> Self {
        let (n, binary_tree) = (data.len(), vec![e; 2 * data.len().next_power_of_two()]);
        let segment_tree = SegmentTree {
            n,
            f,
            e,
            binary_tree,
        };
        segment_tree.init(data)
    }

    /// O(n) # init segment tree by given data
    fn init(mut self, data: &[T]) -> Self {
        let leaf_offset = self.leaf_offset();
        for (i, &di) in data.iter().enumerate() {
            self.binary_tree[leaf_offset + i] = di;
        }
        for i in (1..leaf_offset).rev() {
            self.binary_tree[i] = (self.f)(self.binary_tree[i * 2], self.binary_tree[i * 2 + 1]);
        }
        self
    }

    /// O(1) # get beginning index of the segment tree leaf
    fn leaf_offset(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// O(1) # get size of leaves
    fn num_of_leaf(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// O(log(n)) # set leaf[k] = x, and update segment tree. (non-recursive)
    fn update(&mut self, k: usize, x: T) {
        let mut current = self.leaf_offset() + k;
        self.binary_tree[current] = x;
        while current / 2 > 0 {
            current /= 2;
            self.binary_tree[current] = (self.f)(
                self.binary_tree[current * 2],
                self.binary_tree[current * 2 + 1],
            );
        }
    }

    /// O(log(n)) # calculate f(l, l+1, ..., r-1). note the half interval [l, r). (non-recursive)
    fn query(&self, l: usize, r: usize) -> T {
        let (mut l, mut r) = (self.leaf_offset() + l, self.leaf_offset() + r);
        let mut result = self.e;
        while l < r {
            if l % 2 == 1 {
                result = (self.f)(result, self.binary_tree[l]);
                l += 1; // l is right child, so move to next subtree.
            }
            if r % 2 == 1 {
                result = (self.f)(result, self.binary_tree[r ^ 1]);
            }
            l /= 2;
            r /= 2;
        }
        result
    }

    fn bisect_left<F>(&self, l: usize, r: usize, cmp: F) -> Option<usize>
    where
        F: Fn(T) -> bool,
    {
        self.bisect_left_recursive(l, r, &cmp, 1, 0, self.num_of_leaf())
    }

    fn bisect_left_recursive<F>(
        &self,
        l: usize,
        r: usize,
        cmp: &F,
        node: usize,
        from: usize,
        to: usize,
    ) -> Option<usize>
    where
        F: Fn(T) -> bool,
    {
        if to - from <= 1 {
            if l <= from && to <= r {
                Some(from)
            } else {
                None
            }
        } else if from <= r || l <= to {
            let mid = (from + to) / 2;
            let mut result = None;
            if cmp(self.binary_tree[node * 2]) {
                result = self.bisect_left_recursive(l, r, cmp, node * 2, from, mid);
            }
            if result == None && cmp(self.binary_tree[node * 2 + 1]) {
                result = self.bisect_left_recursive(l, r, cmp, node * 2 + 1, mid, to);
            }
            result
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_test() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(
            t.binary_tree,
            vec![0, 40, 27, 13, 12, 15, 13, 0, 10, 2, 3, 12, 13, 0, 0, 0]
        );
        t.update(3, 22);
        assert_eq!(
            t.binary_tree,
            vec![0, 50, 37, 13, 12, 25, 13, 0, 10, 2, 3, 22, 13, 0, 0, 0]
        );
    }

    #[test]
    fn query_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7];
        let t = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(t.query(2, 6), 14);
        assert_eq!(t.query(2, 5), 9);
        assert_eq!(t.query(3, 5), 7);
        assert_eq!(t.query(3, 6), 12);
    }

    #[test]
    fn query_bound_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7];
        let t = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
        assert_eq!(t.query(9, 3), 0);
    }

    #[test]
    fn update_query_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut t = SegmentTree::new(&data, 0, |a, b| a + b);
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
        let mut t = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(t.query(0, 1), 4);
        assert_eq!(t.query(0, 0), 0);
        t.update(0, 100);
        assert_eq!(t.query(0, 1), 100);
        assert_eq!(t.query(0, 0), 0);
    }

    #[test]
    fn minimum0_tree_test() {
        let data = Vec::<usize>::new();
        let t = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
    }

    #[test]
    fn bisect_left_test() {
        let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::new(&data, std::i32::MIN, |a, b| a.max(b));
        assert_eq!(max_tree.bisect_left(2, 5, |x| x >= 10), Some(2));
        assert_eq!(max_tree.bisect_left(3, 5, |x| x >= 10), None);
        max_tree.update(2, -5);
        assert_eq!(max_tree.bisect_left(1, 3, |x| x >= -5), Some(1));
        assert_eq!(max_tree.bisect_left(1, 5, |x| x >= 500), None);
        assert_eq!(max_tree.bisect_left(5, 10, |x| x >= 500), Some(7));
    }

    #[test]
    fn sum_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut sum_tree = SegmentTree::new(&data, 0, |a, b| a + b);
        assert_eq!(sum_tree.query(3, 5), 7);
        assert_eq!(sum_tree.query(2, 7), 20);
        assert_eq!(sum_tree.query(0, 11), 55);
        sum_tree.update(5, 10);
        assert_eq!(sum_tree.query(3, 5), 7);
        assert_eq!(sum_tree.query(2, 7), 25);
        assert_eq!(sum_tree.query(0, 11), 60);
    }

    #[test]
    fn product_test() {
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut product_tree = SegmentTree::new(&data, 1, |a, b| a * b);
        assert_eq!(product_tree.query(3, 5), 12);
        assert_eq!(product_tree.query(2, 7), 720);
        assert_eq!(product_tree.query(0, 11), 0);
        product_tree.update(5, 10);
        assert_eq!(product_tree.query(3, 5), 12);
        assert_eq!(product_tree.query(2, 7), 1440);
        assert_eq!(product_tree.query(0, 11), 0);
    }

    #[test]
    fn max_test() {
        let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::new(&data, std::i32::MIN, |a, b| a.max(b));
        assert_eq!(max_tree.query(3, 5), -12);
        assert_eq!(max_tree.query(2, 7), 122);
        assert_eq!(max_tree.query(0, 10), 500);
        max_tree.update(5, 1000);
        assert_eq!(max_tree.query(3, 5), -12);
        assert_eq!(max_tree.query(2, 7), 1000);
        assert_eq!(max_tree.query(0, 10), 1000);
    }

    #[test]
    fn min_test() {
        let data = [2, -5, 122, 33, 12, 14, -55, 500, 3];
        let mut min_tree = SegmentTree::new(&data, std::i32::MAX, |a, b| a.min(b));
        assert_eq!(min_tree.query(3, 5), 12);
        assert_eq!(min_tree.query(2, 7), -55);
        assert_eq!(min_tree.query(0, 10), -55);
        min_tree.update(5, -1000);
        assert_eq!(min_tree.query(3, 5), 12);
        assert_eq!(min_tree.query(2, 7), -1000);
        assert_eq!(min_tree.query(0, 10), -1000);
    }

    #[test]
    fn gcd_test() {
        use num::integer::gcd;
        let data = [10, 3, 4, 8, 6, 2];
        let mut gcd_tree = SegmentTree::new(&data, 0, |a, b| gcd(a, b));
        assert_eq!(gcd_tree.query(2, 4), 4);
        assert_eq!(gcd_tree.query(2, 6), 2);
        assert_eq!(gcd_tree.query(0, 6), 1);
        gcd_tree.update(5, 7);
        assert_eq!(gcd_tree.query(2, 4), 4);
        assert_eq!(gcd_tree.query(2, 6), 1);
        assert_eq!(gcd_tree.query(0, 6), 1);
    }

    #[test]
    fn lcm_test() {
        use num::integer::lcm;
        let data = [10, 3, 4, 8, 6, 2];
        let mut lcm_tree = SegmentTree::new(&data, 1, |a, b| lcm(a, b));
        assert_eq!(lcm_tree.query(2, 4), 8);
        assert_eq!(lcm_tree.query(2, 6), 24);
        assert_eq!(lcm_tree.query(0, 6), 120);
        lcm_tree.update(5, 7);
        assert_eq!(lcm_tree.query(2, 4), 8);
        assert_eq!(lcm_tree.query(2, 6), 168);
        assert_eq!(lcm_tree.query(0, 6), 840);
    }

    #[test]
    fn xor_test() {
        let data = [0b111, 0b101, 0b100, 0b000, 0b010];
        let mut xor_tree = SegmentTree::new(&data, 0, |a, b| a ^ b);
        assert_eq!(xor_tree.query(2, 4), 0b100);
        assert_eq!(xor_tree.query(2, 5), 0b110);
        assert_eq!(xor_tree.query(0, 5), 0b100);
        xor_tree.update(4, 0b110);
        assert_eq!(xor_tree.query(2, 4), 0b100);
        assert_eq!(xor_tree.query(2, 5), 0b010);
        assert_eq!(xor_tree.query(0, 5), 0b000);
    }
}
