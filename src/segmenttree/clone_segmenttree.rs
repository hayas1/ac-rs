#![allow(dead_code)]

struct SegmentTree<T: Clone> {
    n: usize,
    f: fn(T, T) -> T,
    e: T,
    binary_tree: Vec<T>,
}
impl<T: Clone> SegmentTree<T> {
    /// O(n) # create segment tree (e is identity element for a function f in type T)
    fn new(data: &[T], e: T, f: fn(T, T) -> T) -> Self {
        let (n, e, binary_tree) = (
            data.len(),
            e.clone(),
            vec![e.clone(); 2 * data.len().next_power_of_two() - 1],
        );
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
        for (i, di) in data.iter().enumerate() {
            self.binary_tree[leaf_offset + i] = di.clone();
        }
        for i in (0..leaf_offset).rev() {
            self.binary_tree[i] = (self.f)(
                self.binary_tree[Self::left_child(i)].clone(),
                self.binary_tree[Self::right_child(i)].clone(),
            );
        }
        self
    }

    /// O(1) # get beginning index of the segment tree leaf
    fn leaf_offset(&self) -> usize {
        self.n.next_power_of_two() - 1
    }

    /// O(1) # get size of leaves
    fn num_of_leaf(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// O(1) # get left child index of node x
    fn left_child(x: usize) -> usize {
        x * 2 + 1
    }

    /// O(1) # get right child index of node x
    fn right_child(x: usize) -> usize {
        x * 2 + 2
    }

    /// O(1) # get parent index of node x
    fn parent(x: usize) -> usize {
        (x - 1) / 2
    }

    /// O(1) # get root index
    fn root() -> usize {
        0
    }

    /// O(1) # either node x is root or not
    fn is_root(x: usize) -> bool {
        x == Self::root()
    }

    /// O(log(n)) # update segment tree, leaf[k] = x
    fn update(&mut self, k: usize, x: T) {
        let i = self.leaf_offset() + k;
        self.binary_tree[i] = x;
        if !Self::is_root(i) {
            self.recursive_update(Self::parent(i));
        }
    }

    /// O(log(i)) # update from leaf to root
    fn recursive_update(&mut self, i: usize) {
        self.binary_tree[i] = (self.f)(
            self.binary_tree[Self::left_child(i)].clone(),
            self.binary_tree[Self::right_child(i)].clone(),
        );
        if !Self::is_root(i) {
            self.recursive_update(Self::parent(i));
        }
    }

    /// O(log(n)) # calculate half-open interval summation from l to r (leaf index)
    fn query(&self, l: usize, r: usize) -> T {
        self.recursive_query(l, r, Self::root(), 0, self.num_of_leaf())
    }

    /// O(log(n)-log(node)) # calculate summation from root to leaf
    fn recursive_query(&self, l: usize, r: usize, node: usize, from: usize, to: usize) -> T {
        if r <= from || to <= l {
            self.e.clone()
        } else if l <= from && to <= r {
            self.binary_tree[node].clone()
        } else {
            let mid = (from + to) / 2;
            (self.f)(
                self.recursive_query(l, r, Self::left_child(node), from, mid),
                self.recursive_query(l, r, Self::right_child(node), mid, to),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_test() {
        let data: Vec<_> = ["r", "u", "s", "t", "a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut t = SegmentTree::new(&data, "".to_string(), |a, b| format!("{}{}", a, b));
        assert_eq!(t.query(0, 4), "rust");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "us");
        assert_eq!(t.query(0, 0), "");
        t.update(2, "b".to_string());
        t.update(3, "y".to_string());
        assert_eq!(t.query(0, 4), "ruby");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "ub");
        assert_eq!(t.query(0, 0), "");
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
