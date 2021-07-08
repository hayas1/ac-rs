use std::ops::{Bound, RangeBounds};
pub trait Monoid<T> {
    /// wrap input to Monoid
    fn from(a: T) -> Self;
    /// unwrap from monoid
    fn into(self) -> T;
    /// identity element of Monoid
    fn identity() -> Self;
    /// binary operation that satisfy associative law for Monoid
    fn operation(a: &Self, b: &Self) -> Self;
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
    pub fn update<T>(&mut self, i: usize, x: T) -> M
    where
        M: Monoid<T> + Clone,
    {
        self.update_with(i, |_| x)
    }

    /// **O(log(n))**, update leaf[k] by f(leaf[k]), and update segment tree. (non-recursive)
    pub fn update_with<T, U>(&mut self, i: usize, f: U) -> M
    where
        M: Monoid<T> + Clone,
        U: FnOnce(&T) -> T,
    {
        assert!(i < self.len(), "index {} is out of 0..{}", i, self.len());
        let mut node = self.leaf_offset() + i;
        let mut result = M::from(f(&self.tree[node].clone().into()));
        std::mem::swap(&mut self.tree[node], &mut result);
        while node > 0 {
            node = (node - 1) / 2;
            self.tree[node] = M::operation(&self.tree[node * 2 + 1], &self.tree[node * 2 + 2]);
        }
        result
    }

    /// **O(1)**, range to leaf index half interval [left, right).
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
        assert!(left <= right);
        (left, right)
    }

    /// **O(log(n))**, calculate f(range). (non-recursive)
    pub fn query<T, R>(&self, range: R) -> M
    where
        M: Monoid<T> + Clone,
        R: RangeBounds<usize>,
    {
        let (left, right) = self.indices(range);
        let (mut left, mut right) = (self.leaf_offset() + left, self.leaf_offset() + right);
        let (mut left_result, mut right_result) = (M::identity(), M::identity());
        while left < right {
            if left % 2 == 0 {
                // l is right child
                left_result = M::operation(&left_result, &self.tree[left]);
                left += 1; //  move to next subtree.
            }
            if right % 2 == 0 {
                // r is right child
                right_result = M::operation(&self.tree[right - 1], &right_result);
            }
            left = (left - 1) / 2;
            right = (right - 1) / 2;
        }
        M::operation(&left_result, &right_result)
    }

    /// **O(log^2(n))**, search the leaf where cmp(x) is true in half interval [l, r).
    pub fn bisect<T, R, F>(&self, range: R, cmp: F, leftmost: bool) -> Option<usize>
    where
        R: RangeBounds<usize>,
        M: Monoid<T> + Clone,
        F: Fn(&T) -> bool,
    {
        let (mut from, mut to) = self.indices(range);
        while to - from > 1 {
            let mid = (from + to) / 2;
            // bisect_right and bisect_left is merged into one function, so code is bad...
            let (left_cmp, right_cmp) =
                (cmp(&self.query(from..mid).into()), cmp(&self.query(mid..to).into()));
            if leftmost && left_cmp || !leftmost && !right_cmp {
                to = mid;
            } else if leftmost && !left_cmp || !leftmost && right_cmp {
                from = mid;
            } else {
                unreachable!();
            }
        }
        if cmp(&self.tree[self.leaf_offset() + from].clone().into()) {
            Some(from)
        } else {
            None
        }
    }
}
