use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

pub struct SegmentTree<T, F, E>
where
    F: Fn(&T, &T) -> T,
    E: Fn() -> T,
{
    n: usize,            // size of data
    f: F,                // binary operation of the monoid
    e: E,                // identity element of the monoid
    binary_tree: Vec<T>, // 1-indexed binary tree (parent: x/2, left_child: x*2, right_child: x*2+1, sibling: x^1, is_x_left_child: x%2==0, is_x_right_child: x%2==1)
}
impl<T, F, E, I: SliceIndex<[T]>> Index<I> for SegmentTree<T, F, E>
where
    F: Fn(&T, &T) -> T,
    E: Fn() -> T,
{
    type Output = I::Output;
    /// **O(n)...?** this function might make size n temporary slice
    fn index(&self, index: I) -> &Self::Output {
        &self.binary_tree[self.leaf_offset()..self.leaf_offset() + self.n][index]
    }
}
impl<T, F, E, I: SliceIndex<[T]>> IndexMut<I> for SegmentTree<T, F, E>
where
    F: Fn(&T, &T) -> T,
    E: Fn() -> T,
{
    /// **O(n)...?** this function might make size n temporary slice
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let lol = self.leaf_offset();
        let lor = lol + self.n;
        &mut self.binary_tree[lol..lor][index]
    }
}
impl<T, F, E> SegmentTree<T, F, E>
where
    F: Fn(&T, &T) -> T,
    E: Fn() -> T,
{
    /// **O(n)**, create segment tree, note that this method requires ownership of data.
    pub fn from(data: Vec<T>, e: E, f: F) -> Self {
        let (n, binary_tree) = (
            data.len(),
            (0..2 * data.len().next_power_of_two())
                .map(|_| e())
                .collect(),
        );
        let segment_tree = SegmentTree {
            n,
            f,
            e,
            binary_tree,
        };
        segment_tree.init(data)
    }

    /// **O(n)**, init segment tree by given data.
    fn init(mut self, data: Vec<T>) -> Self {
        let leaf_offset = self.leaf_offset();
        for (i, di) in data.into_iter().enumerate() {
            self.binary_tree[leaf_offset + i] = di;
        }
        for i in (1..leaf_offset).rev() {
            self.binary_tree[i] = (self.f)(&self.binary_tree[i * 2], &self.binary_tree[i * 2 + 1]);
        }
        self
    }

    /// **O(1)**, get beginning index of the segment tree leaf.
    pub fn leaf_offset(&self) -> usize {
        self.n.next_power_of_two()
    }

    /// **O(log(n))**, set leaf[k] = x, and update segment tree. (non-recursive)
    pub fn update(&mut self, k: usize, x: T) -> T {
        self.update_with(k, |_| x)
    }

    /// **O(log(n))**, update leaf[k] by f(leaf[k]), and update segment tree. (non-recursive)
    pub fn update_with<U>(&mut self, k: usize, f: U) -> T
    where
        U: FnOnce(&T) -> T,
    {
        let new_value = f(&self[k]);
        let present = std::mem::replace(&mut self[k], new_value);
        self.update_parents(k);
        present
    }

    /// **O(log(n))**, swap leaf[k] and leaf[l], and update segment tree. (non-recursive)
    pub fn swap(&mut self, k: usize, l: usize) {
        let (lok, lol) = (self.leaf_offset() + k, self.leaf_offset() + l);
        self.binary_tree.swap(lok, lol);
        self.update_parents(k);
        self.update_parents(l);
    }

    /// **O(log(n))**, update segment tree. (non-recursive)
    pub fn update_parents(&mut self, k: usize) {
        let mut current = self.leaf_offset() + k;
        while current / 2 > 0 {
            current /= 2;
            self.binary_tree[current] = (self.f)(
                &self.binary_tree[current * 2],
                &self.binary_tree[current * 2 + 1],
            );
        }
    }

    /// **O(log(n))**, calculate f(l, l+1, ..., r-1). note the half interval [l, r). (non-recursive)
    pub fn query(&self, l: usize, r: usize) -> T {
        let (mut li, mut ri) = (self.leaf_offset() + l, self.leaf_offset() + r);
        let (mut result_left, mut result_right) = ((self.e)(), (self.e)());
        while li < ri {
            if li % 2 == 1 {
                result_left = (self.f)(&result_left, &self.binary_tree[li]);
                li += 1; // l is right child, so move to next subtree.
            }
            if ri % 2 == 1 {
                result_right = (self.f)(&self.binary_tree[ri ^ 1], &result_right);
            }
            li /= 2;
            ri /= 2;
        }
        (self.f)(&result_left, &result_right)
    }

    /// **O(log^2(n))**, search the leftmost leaf where cmp(x) is true in half interval [l, r).
    pub fn bisect_left<C>(&self, l: usize, r: usize, cmp: C) -> Option<usize>
    where
        C: Fn(&T) -> bool,
    {
        let (mut from, mut to) = (l, r);
        while to - from > 1 {
            let mid = (from + to) / 2;
            if cmp(&self.query(from, mid)) {
                to = mid;
            } else {
                from = mid;
            }
        }
        if cmp(&self.binary_tree[self.leaf_offset() + from]) {
            Some(from)
        } else {
            None
        }
    }

    /// **O(log^2(n))**, search the rightmost leaf where cmp(x) is true in half interval [l, r).
    pub fn bisect_right<C>(&self, l: usize, r: usize, cmp: C) -> Option<usize>
    where
        C: Fn(&T) -> bool,
    {
        let (mut from, mut to) = (l, r);
        while to - from > 1 {
            let mid = (from + to) / 2;
            if cmp(&self.query(mid, to)) {
                from = mid;
            } else {
                to = mid;
            }
        }
        if cmp(&self.binary_tree[self.leaf_offset() + from]) {
            Some(from)
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
        let mut t = SegmentTree::from(data, || 0, |&a, &b| a + b);
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
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let t = SegmentTree::from(data, || 0, |&a, &b| a + b);
        assert_eq!(t.query(2, 6), 14);
        assert_eq!(t.query(2, 5), 9);
        assert_eq!(t.query(3, 5), 7);
        assert_eq!(t.query(3, 6), 12);
    }

    #[test]
    fn swap_test() {
        let data = vec![10, 2, 3, 12, 13];
        let mut t = SegmentTree::from(data, || 0, |a, b| a + b);
        assert_eq!(
            t.binary_tree,
            vec![0, 40, 27, 13, 12, 15, 13, 0, 10, 2, 3, 12, 13, 0, 0, 0]
        );
        t.swap(0, 4);
        assert_eq!(
            t.binary_tree,
            vec![0, 40, 30, 10, 15, 15, 10, 0, 13, 2, 3, 12, 10, 0, 0, 0]
        )
    }

    #[test]
    fn index_test() {
        let data = vec![10, 2, 3, 12, 13];
        let t = SegmentTree::from(data, || 0, |a, b| a + b);
        assert_eq!(t[..], [10, 2, 3, 12, 13]);
        assert_eq!(t[0..], [10, 2, 3, 12, 13]);
        assert_eq!(t[..5], [10, 2, 3, 12, 13]);
        assert_eq!(t[0..2], [10, 2]);
        assert_eq!(t[2..3], [3]);
        assert_eq!(t[2], 3);
        assert_eq!(t[2..2], []);
    }

    #[test]
    fn query_bound_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let t = SegmentTree::from(data, || 0, |&a, &b| a + b);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
        assert_eq!(t.query(9, 3), 0);
    }

    #[test]
    fn update_query_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut t = SegmentTree::from(data, || 0, |&a, &b| a + b);
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
        let mut t = SegmentTree::from(data, || 0, |&a, &b| a + b);
        assert_eq!(t.query(0, 1), 4);
        assert_eq!(t.query(0, 0), 0);
        t.update(0, 100);
        assert_eq!(t.query(0, 1), 100);
        assert_eq!(t.query(0, 0), 0);
    }

    #[test]
    fn minimum0_tree_test() {
        let data = Vec::<usize>::new();
        let t = SegmentTree::from(data, || 0, |&a, &b| a + b);
        assert_eq!(t.query(0, 1), 0);
        assert_eq!(t.query(0, 0), 0);
    }

    #[test]
    fn bisect_left_test() {
        let data = vec![2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::from(data, || std::i32::MIN, |&a, &b| a.max(b));
        assert_eq!(max_tree.bisect_left(2, 5, |&x| x >= 10), Some(2));
        assert_eq!(max_tree.bisect_left(3, 5, |&x| x >= 10), None);
        max_tree.update(2, -5);
        assert_eq!(max_tree.bisect_left(1, 3, |&x| x >= -5), Some(1));
        assert_eq!(max_tree.bisect_left(1, 5, |&x| x >= 500), None);
        assert_eq!(max_tree.bisect_left(5, 10, |&x| x >= 500), Some(7));
    }

    #[test]
    fn bisect_right_test() {
        let data = vec![2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::from(data, || std::i32::MIN, |&a, &b| a.max(b));
        assert_eq!(max_tree.bisect_right(2, 5, |&x| x >= 10), Some(2));
        assert_eq!(max_tree.bisect_right(3, 5, |&x| x >= 10), None);
        max_tree.update(2, -5);
        assert_eq!(max_tree.bisect_right(1, 3, |&x| x >= -5), Some(2));
        assert_eq!(max_tree.bisect_right(1, 5, |&x| x >= 500), None);
        assert_eq!(max_tree.bisect_right(5, 10, |&x| x >= 500), Some(7));
        max_tree.update(3, -5);
        assert_eq!(max_tree.bisect_right(1, 5, |&x| x >= -5), Some(3));
        max_tree.update(4, -5);
        assert_eq!(max_tree.bisect_right(1, 5, |&x| x >= -5), Some(4));
    }

    #[test]
    fn sum_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut sum_tree = SegmentTree::from(data, || 0, |&a, &b| a + b);
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
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut product_tree = SegmentTree::from(data, || 1, |&a, &b| a * b);
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
        let data = vec![2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::from(data, || std::i32::MIN, |&a, &b| a.max(b));
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
        let data = vec![2, -5, 122, 33, 12, 14, -55, 500, 3];
        let mut min_tree = SegmentTree::from(data, || std::i32::MAX, |&a, &b| a.min(b));
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
        let data = vec![10, 3, 4, 8, 6, 2];
        let mut gcd_tree = SegmentTree::from(data, || 0, |&a, &b| gcd(a, b));
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
        let data = vec![10, 3, 4, 8, 6, 2];
        let mut lcm_tree = SegmentTree::from(data, || 1, |&a, &b| lcm(a, b));
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
        let data = vec![0b111, 0b101, 0b100, 0b000, 0b010];
        let mut xor_tree = SegmentTree::from(data, || 0, |&a, &b| a ^ b);
        assert_eq!(xor_tree.query(2, 4), 0b100);
        assert_eq!(xor_tree.query(2, 5), 0b110);
        assert_eq!(xor_tree.query(0, 5), 0b100);
        xor_tree.update(4, 0b110);
        assert_eq!(xor_tree.query(2, 4), 0b100);
        assert_eq!(xor_tree.query(2, 5), 0b010);
        assert_eq!(xor_tree.query(0, 5), 0b000);
    }

    #[test]
    fn join_test() {
        let data = "rustabc";
        let mut t = SegmentTree::from(
            data.split("").skip(1).map(|s| s.to_string()).collect(), // first element is ""
            || "".to_string(),
            |a, b| format!("{}{}", a, b),
        );
        assert_eq!(t.query(0, 4), "rust");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "us");
        assert_eq!(t.query(0, 1), "r");
        assert_eq!(t.query(0, 0), "");
        t.update(2, "b".to_string());
        t.update(3, "y".to_string());
        assert_eq!(t.query(0, 4), "ruby");
        assert_eq!(t.query(4, 7), "abc");
        assert_eq!(t.query(1, 3), "ub");
        assert_eq!(t.query(0, 0), "");
    }
}
