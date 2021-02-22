use std::ops::{Bound, RangeBounds};
pub trait Monoid<T>: From<T> + Into<T> {
    /// identity element of Monoid
    fn identity() -> Self;
    /// binary operation for Monoid
    fn operation(a: &Self, b: &Self) -> Self;
}

pub enum BisectDirection {
    Left,
    Right,
}

pub struct SegmentTree<M> {
    len: usize,
    tree: Vec<M>, // 0-indexed perfect binary tree
}
impl<M> SegmentTree<M> {
    /// **O(n)**, create segment tree. (e is identity element for a function f in type T)
    pub fn new<T>(data: &[T]) -> Self
    where
        T: Clone,
        M: Monoid<T> + Clone,
    {
        let binary_tree = vec![M::identity(); 2 * data.len().next_power_of_two() + 1];
        let segment_tree = SegmentTree { len: data.len(), tree: binary_tree };
        segment_tree.init(data)
    }

    /// **O(n)**, init segment tree by given data.
    fn init<T>(mut self, data: &[T]) -> Self
    where
        T: Clone,
        M: Monoid<T> + Clone,
    {
        let leaf_offset = self.leaf_offset();
        for (i, di) in data.iter().enumerate() {
            self.tree[leaf_offset + i] = M::from(di.clone());
        }
        for i in (1..leaf_offset).rev() {
            self.tree[i] = M::operation(&self.tree[i * 2 + 1], &self.tree[i * 2 + 2]);
        }
        self
    }

    /// **O(1)**, return this segtree 's number of data
    pub fn len(&self) -> usize {
        self.len
    }

    /// **O(1)**, get beginning index of the segment tree leaf.
    pub fn leaf_offset(&self) -> usize {
        self.len().next_power_of_two() - 1
    }

    /// **O(log(n))**, set leaf[k] = x, and update segment tree. (non-recursive)
    pub fn update<T>(&mut self, i: usize, x: T) -> T
    where
        M: Monoid<T> + Clone,
    {
        self.update_with(i, |_| x)
    }

    /// **O(log(n))**, update leaf[k] by f(leaf[k]), and update segment tree. (non-recursive)
    pub fn update_with<T, U>(&mut self, i: usize, f: U) -> T
    where
        M: Monoid<T> + Clone,
        U: FnOnce(&T) -> T,
    {
        assert!(i < self.len(), "index {} is out of 0..{}", i, self.len());
        let mut node = self.leaf_offset() + i;
        let mut result = M::from(f(&self.tree[node].clone().into()));
        std::mem::swap(&mut self.tree[node], &mut result);
        while (node - 1) / 2 > 0 {
            node = (node - 1) / 2;
            self.tree[node] = M::operation(&self.tree[node * 2 + 1], &self.tree[node * 2 + 2]);
        }
        result.into()
    }

    /// **O(1)**, range to half interval [left, right).
    pub fn indices<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        let left = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&l) => (l + 1).max(0),
            Bound::Included(&l) => l.max(0),
        };
        let right = match range.end_bound() {
            Bound::Unbounded => self.len(),
            Bound::Excluded(&r) => r.min(self.len()),
            Bound::Included(&r) => (r + 1).min(self.len()),
        };
        (left, right)
    }

    /// **O(log(n))**, calculate f(range). (non-recursive)
    pub fn query<T, R>(&self, range: R) -> T
    where
        M: Monoid<T> + Clone,
        R: RangeBounds<usize>,
    {
        let (left, right) = self.indices(range);
        let (mut left, mut right) = (self.leaf_offset() + left, self.leaf_offset() + right);
        let mut result = M::identity();
        while left < right {
            if left % 2 == 0 {
                result = M::operation(&result, &self.tree[left]); // l is right child
                left += 1; //  move to next subtree.
            }
            if right % 2 == 0 {
                result = M::operation(&result, &self.tree[right - 1]); // r is right child
            }
            left = (left - 1) / 2;
            right = (right - 1) / 2;
        }
        result.into()
    }

    /// **O(log^2(n))**, search the leaf where cmp(x) is true in half interval [l, r).
    pub fn bisect<T, R, F>(&self, range: R, cmp: F, bi: BisectDirection) -> Option<usize>
    where
        R: RangeBounds<usize>,
        M: Monoid<T> + Clone,
        F: Fn(&T) -> bool,
    {
        let (mut from, mut to) = self.indices(range);
        while to - from > 1 {
            let mid = (from + to) / 2;
            if match bi {
                BisectDirection::Left => cmp(&self.query(from..mid)),
                BisectDirection::Right => cmp(&self.query(mid..to)),
            } {
                match bi {
                    BisectDirection::Left => to = mid,
                    BisectDirection::Right => from = mid,
                }
            } else {
                match bi {
                    BisectDirection::Left => from = mid,
                    BisectDirection::Right => to = mid,
                }
            }
        }
        if cmp(&self.tree[self.leaf_offset() + from].clone().into()) {
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
    fn empty_tree_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Sum(usize);
        impl Monoid<usize> for Sum {
            fn identity() -> Self {
                Self(0)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Self(a.0 + b.0)
            }
        }
        impl From<usize> for Sum {
            fn from(a: usize) -> Self {
                Self(a)
            }
        }
        impl Into<usize> for Sum {
            fn into(self) -> usize {
                self.0
            }
        }
        let sum_tree = SegmentTree::<Sum>::new(&[]);
        assert_eq!(sum_tree.query(..), 0);
    }

    #[test]
    fn bisect_left_right_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Max(i64);
        impl Monoid<i64> for Max {
            fn identity() -> Self {
                Self(std::i64::MIN)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Self(std::cmp::max(a.0, b.0))
            }
        }
        impl From<i64> for Max {
            fn from(a: i64) -> Self {
                Self(a)
            }
        }
        impl Into<i64> for Max {
            fn into(self) -> i64 {
                self.0
            }
        }
        let data1 = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree1 = SegmentTree::<Max>::new(&data1);
        assert_eq!(max_tree1.bisect(2..5, |&x| x >= 10, BisectDirection::Left), Some(2));
        assert_eq!(max_tree1.bisect(3..5, |&x| x >= 10, BisectDirection::Left), None);
        max_tree1.update(2, -5);
        assert_eq!(max_tree1.bisect(1..3, |&x| x >= -5, BisectDirection::Left), Some(1));
        assert_eq!(max_tree1.bisect(1..5, |&x| x >= 500, BisectDirection::Left), None);
        assert_eq!(max_tree1.bisect(5..10, |&x| x >= 500, BisectDirection::Left), Some(7));
        let data2 = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree2 = SegmentTree::<Max>::new(&data2);
        assert_eq!(max_tree2.bisect(2..5, |&x| x >= 10, BisectDirection::Right), Some(2));
        assert_eq!(max_tree2.bisect(3..5, |&x| x >= 10, BisectDirection::Right), None);
        max_tree2.update(2, -5);
        assert_eq!(max_tree2.bisect(1..3, |&x| x >= -5, BisectDirection::Right), Some(2));
        assert_eq!(max_tree2.bisect(1..5, |&x| x >= 500, BisectDirection::Right), None);
        assert_eq!(max_tree2.bisect(5..10, |&x| x >= 500, BisectDirection::Right), Some(7));
        max_tree2.update(3, -5);
        assert_eq!(max_tree2.bisect(1..5, |&x| x >= -5, BisectDirection::Right), Some(3));
        max_tree2.update(4, -5);
        assert_eq!(max_tree2.bisect(1..5, |&x| x >= -5, BisectDirection::Right), Some(4));
    }

    #[test]
    fn sum_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Sum(u64);
        impl Monoid<u64> for Sum {
            fn identity() -> Self {
                Self(0)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Self(a.0 + b.0)
            }
        }
        impl From<u64> for Sum {
            fn from(a: u64) -> Self {
                Self(a)
            }
        }
        impl Into<u64> for Sum {
            fn into(self) -> u64 {
                self.0
            }
        }
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut sum_tree = SegmentTree::<Sum>::new(&data);
        assert_eq!(sum_tree.query(3..5), 7);
        assert_eq!(sum_tree.query(2..7), 20);
        assert_eq!(sum_tree.query(..), 55);
        sum_tree.update(5, 10);
        assert_eq!(sum_tree.query(3..=4), 7);
        assert_eq!(sum_tree.query(2..7), 25);
        assert_eq!(sum_tree.query(1..), 60);
        sum_tree.update_with(7, |x| x * 2); // t[7] = 14
        assert_eq!(sum_tree.query(..6), 20);
        assert_eq!(sum_tree.query(6..=8), 28);
    }

    #[test]
    fn prod_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Prod(u64);
        impl Monoid<u64> for Prod {
            fn identity() -> Self {
                Prod(1)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Prod(a.0 * b.0)
            }
        }
        impl From<u64> for Prod {
            fn from(a: u64) -> Self {
                Self(a)
            }
        }
        impl Into<u64> for Prod {
            fn into(self) -> u64 {
                self.0
            }
        }
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut prod_tree = SegmentTree::<Prod>::new(&data);
        assert_eq!(prod_tree.query(3..5), 12);
        assert_eq!(prod_tree.query(2..7), 720);
        assert_eq!(prod_tree.query(0..11), 0);
        prod_tree.update(5, 10);
        assert_eq!(prod_tree.query(3..5), 12);
        assert_eq!(prod_tree.query(2..7), 1440);
        assert_eq!(prod_tree.query(0..), 0);
        prod_tree.update_with(7, |x| x / 2); // t[7] = 3
        assert_eq!(prod_tree.query(5..=7), 180);
        assert_eq!(prod_tree.query(8..), 720);
    }

    #[test]
    fn max_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Max(i64);
        impl Monoid<i64> for Max {
            fn identity() -> Self {
                Self(std::i64::MIN)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Self(std::cmp::max(a.0, b.0))
            }
        }
        impl From<i64> for Max {
            fn from(a: i64) -> Self {
                Self(a)
            }
        }
        impl Into<i64> for Max {
            fn into(self) -> i64 {
                self.0
            }
        }
        let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
        let mut max_tree = SegmentTree::<Max>::new(&data);
        assert_eq!(max_tree.query(3..5), -12);
        assert_eq!(max_tree.query(2..=6), 122);
        assert_eq!(max_tree.query(..), 500);
        max_tree.update(5, 1000);
        assert_eq!(max_tree.query(3..=4), -12);
        assert_eq!(max_tree.query(2..7), 1000);
        assert_eq!(max_tree.query(..10), 1000);
    }

    #[test]
    fn min_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Min(i64);
        impl Monoid<i64> for Min {
            fn identity() -> Self {
                Min(std::i64::MAX)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Min(std::cmp::min(a.0, b.0))
            }
        }
        impl From<i64> for Min {
            fn from(a: i64) -> Self {
                Self(a)
            }
        }
        impl Into<i64> for Min {
            fn into(self) -> i64 {
                self.0
            }
        }
        let data = [2, -5, 122, 33, 12, 14, -55, 500, 3];
        let mut min_tree = SegmentTree::<Min>::new(&data);
        assert_eq!(min_tree.query(3..5), 12);
        assert_eq!(min_tree.query(2..7), -55);
        assert_eq!(min_tree.query(0..), -55);
        min_tree.update(5, -1000);
        assert_eq!(min_tree.query(3..5), 12);
        assert_eq!(min_tree.query(2..7), -1000);
        assert_eq!(min_tree.query(..10), -1000);
    }

    #[test]
    fn gcd_test() {
        use num::integer::gcd;
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Gcd(u64);
        impl Monoid<u64> for Gcd {
            fn identity() -> Self {
                Gcd(0)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Gcd(gcd(a.0, b.0))
            }
        }
        impl From<u64> for Gcd {
            fn from(a: u64) -> Self {
                Self(a)
            }
        }
        impl Into<u64> for Gcd {
            fn into(self) -> u64 {
                self.0
            }
        }
        let data = [10, 3, 4, 8, 6, 2];
        let mut gcd_tree = SegmentTree::<Gcd>::new(&data);
        assert_eq!(gcd_tree.query(2..4), 4);
        assert_eq!(gcd_tree.query(2..6), 2);
        assert_eq!(gcd_tree.query(0..6), 1);
        gcd_tree.update(5, 7);
        assert_eq!(gcd_tree.query(2..4), 4);
        assert_eq!(gcd_tree.query(2..6), 1);
        assert_eq!(gcd_tree.query(0..6), 1);
    }

    #[test]
    fn lcm_test() {
        use num::integer::lcm;
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Lcm(u64);
        impl Monoid<u64> for Lcm {
            fn identity() -> Self {
                Lcm(1)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Lcm(lcm(a.0, b.0))
            }
        }
        impl From<u64> for Lcm {
            fn from(a: u64) -> Self {
                Self(a)
            }
        }
        impl Into<u64> for Lcm {
            fn into(self) -> u64 {
                self.0
            }
        }
        let data = [10, 3, 4, 8, 6, 2];
        let mut lcm_tree = SegmentTree::<Lcm>::new(&data);
        assert_eq!(lcm_tree.query(2..4), 8);
        assert_eq!(lcm_tree.query(2..6), 24);
        assert_eq!(lcm_tree.query(0..6), 120);
        lcm_tree.update(5, 7);
        assert_eq!(lcm_tree.query(2..4), 8);
        assert_eq!(lcm_tree.query(2..6), 168);
        assert_eq!(lcm_tree.query(0..6), 840);
    }

    #[test]
    fn xor_test() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Xor(u64);
        impl Monoid<u64> for Xor {
            fn identity() -> Self {
                Xor(0)
            }
            fn operation(a: &Self, b: &Self) -> Self {
                Xor(a.0 ^ b.0)
            }
        }
        impl From<u64> for Xor {
            fn from(a: u64) -> Self {
                Self(a)
            }
        }
        impl Into<u64> for Xor {
            fn into(self) -> u64 {
                self.0
            }
        }
        let data = [0b111, 0b101, 0b100, 0b000, 0b010];
        let mut xor_tree = SegmentTree::<Xor>::new(&data);
        assert_eq!(xor_tree.query(2..4), 0b100);
        assert_eq!(xor_tree.query(2..5), 0b110);
        assert_eq!(xor_tree.query(0..5), 0b100);
        xor_tree.update(4, 0b110);
        assert_eq!(xor_tree.query(2..4), 0b100);
        assert_eq!(xor_tree.query(2..5), 0b010);
        assert_eq!(xor_tree.query(0..5), 0b000);
    }
}
