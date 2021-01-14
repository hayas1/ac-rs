use std::cmp::Ord;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Index;
use std::slice::{Iter, SliceIndex};

/// should use binary search tree
pub struct SortedVec<T: Ord> {
    v: Vec<T>,
}
impl<T: Ord, I: SliceIndex<[T]>> Index<I> for SortedVec<T> {
    type Output = I::Output;
    #[inline]
    /// **O(size(index))**, get the element(s) of at the index of the list
    fn index(&self, index: I) -> &Self::Output {
        &self.v[index]
    }
}
impl<T: Ord> FromIterator<T> for SortedVec<T> {
    #[inline]
    /// **O(n log(n))**, make sorted list from iterator
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<_> = iter.into_iter().collect();
        v.sort();
        SortedVec { v }
    }
}
impl<T: Ord> Extend<T> for SortedVec<T> {
    #[inline]
    /// **O(n + k log(k))**, extend elements (size k)
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // merge like merge sort
        let mut items: Vec<_> = iter.into_iter().collect();
        items.sort();
        let tmp = self.v.split_off(0);
        let mut items_iter = items.into_iter().peekable();
        for vi in tmp {
            while let Some(ii) = items_iter.peek() {
                if &vi > ii {
                    self.v.push(items_iter.next().unwrap());
                } else {
                    break;
                }
            }
            self.v.push(vi);
        }
        self.v.extend(items_iter);
    }
}
impl<T: Ord> SortedVec<T> {
    /// **O(1)**, get empty vec
    pub fn new() -> Self {
        SortedVec { v: Vec::new() }
    }

    /// **O(1)**, get iterator of vec
    pub fn iter(&self) -> Iter<'_, T> {
        self.v.iter()
    }

    /// **O(n)**, insert element
    pub fn insert(&mut self, element: T) {
        match self.v.binary_search(&element) {
            Ok(index) => self.v.insert(index, element),
            Err(index) => self.v.insert(index, element),
        }
    }

    /// **O(k)**, max k elements
    pub fn max_elements<'a>(&'a self, k: usize) -> &'a [T] {
        let size = self.v.len();
        &self.v[size - k..]
    }

    /// **O(k)**, min k elements
    pub fn min_elements<'a>(&'a self, k: usize) -> &'a [T] {
        &self.v[..k]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_vec_index_test() {
        let v = [1, 5, 5, 4, 4, 4, 5, 1];
        let sorted_vec: SortedVec<_> = v.iter().cloned().collect();
        assert_eq!(sorted_vec[..], [1, 1, 4, 4, 4, 5, 5, 5]);
        assert_eq!(sorted_vec[0], 1);
        assert_eq!(sorted_vec[7], 5);
        assert_eq!(sorted_vec[0..0], []);
        assert_eq!(sorted_vec[..3], [1, 1, 4]);
        assert_eq!(sorted_vec[3..7], [4, 4, 5, 5]);
        assert_eq!(sorted_vec[7..], [5]);
    }

    #[test]
    fn sorted_vec_iter_test() {
        let v = [3, 3, 2, 1, 1, 2, 5];
        let sorted_vec: SortedVec<_> = v.iter().cloned().collect();
        let sorted = [1, 1, 2, 2, 3, 3, 5];
        for (i, vi) in sorted_vec.iter().enumerate() {
            assert_eq!(vi, &sorted[i]);
        }
    }

    #[test]
    fn sorted_vec_insert_extend_test() {
        let v = [1, 2, 1, 3, 5, 1, 2, 2];
        let mut sorted_vec: SortedVec<_> = v.iter().cloned().collect();
        sorted_vec.insert(4);
        assert_eq!(sorted_vec[..], [1, 1, 1, 2, 2, 2, 3, 4, 5]);
        sorted_vec.insert(100);
        assert_eq!(sorted_vec[..], [1, 1, 1, 2, 2, 2, 3, 4, 5, 100]);
        sorted_vec.extend(vec![45, 65, 2, 14, 4]);
        assert_eq!(
            sorted_vec[..],
            [1, 1, 1, 2, 2, 2, 2, 3, 4, 4, 5, 14, 45, 65, 100]
        );
        sorted_vec.extend(vec![32, 12, 15, 17, 3, 2, 1, 99]);
        assert_eq!(
            sorted_vec[..],
            [1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 4, 4, 5, 12, 14, 15, 17, 32, 45, 65, 99, 100]
        );
    }

    #[test]
    fn sorted_vec_min_max_test() {
        let mut sorted_vec = SortedVec::new();
        sorted_vec.insert(50);
        sorted_vec.extend(vec![56, 100, 73, 12, 3]);
        assert_eq!(sorted_vec.min_elements(1), [3]);
        assert_eq!(sorted_vec.min_elements(2), [3, 12]);
        assert_eq!(sorted_vec.min_elements(3), [3, 12, 50]);
        assert_eq!(sorted_vec.max_elements(1), [100]);
        assert_eq!(sorted_vec.max_elements(2), [73, 100]);
        assert_eq!(sorted_vec.max_elements(3), [56, 73, 100]);
    }
}
