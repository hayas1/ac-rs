//   data index:  0 1 2 3 4
// cumsum index: 0 1 2 3 4 5

use num::Zero;
use std::ops::{Add, Bound, RangeBounds, Sub};

pub struct CumSum<T> {
    v: Vec<T>,
}
impl<T: Add<Output = T> + Sub<Output = T> + Zero + Copy> CumSum<T> {
    /// **O(n)**, create n+1 size vec for cumsum
    pub fn new(data: &[T]) -> Self {
        let v = [T::zero()]
            .iter()
            .chain(data.iter())
            .scan(T::zero(), |cum, x| {
                *cum = *cum + *x;
                Some(*cum)
            })
            .collect();
        CumSum { v }
    }

    /// **O(1)**, range to index half interval [left, right).
    pub fn indices<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let left = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&l) => (l + 1).max(0),
            Bound::Included(&l) => l.max(0),
        };
        let right = match range.end_bound() {
            Bound::Unbounded => self.v.len() - 1,
            Bound::Excluded(&r) => r.min(self.v.len() - 1),
            Bound::Included(&r) => (r + 1).min(self.v.len() - 1),
        };
        (left, right)
    }

    /// **O(1)**, calculate half-open interval summation r
    pub fn interval_sum<R: RangeBounds<usize>>(&self, range: R) -> T {
        let (left, right) = self.indices(range);
        self.v[right] - self.v[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cumsum_test() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::new(&data);
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }

    #[test]
    fn interval_sum_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::new(&data);
        assert_eq!(cum.interval_sum(0..3), 3);
        assert_eq!(cum.interval_sum(3..6), 12);
        assert_eq!(cum.interval_sum(6..9), 21);
        assert_eq!(cum.interval_sum(6..=9), 30);
        assert_eq!(cum.interval_sum(5..), 45);
        assert_eq!(cum.interval_sum(..10), 45);
        assert_eq!(cum.interval_sum(..=10), 55);
        assert_eq!(cum.interval_sum(..), 55);
        assert_eq!(cum.interval_sum(..11), 55);
    }

    #[test]
    fn interval_sum_bound_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::new(&data);
        assert_eq!(cum.interval_sum(0..0), 0);
        assert_eq!(cum.interval_sum(1..2), 1);
        assert_eq!(cum.interval_sum(3..0), -3);
        assert_eq!(cum.interval_sum(6..2), -14);
        assert_eq!(cum.interval_sum(11..), 0);
        assert_eq!(cum.interval_sum(11..0), -55);
    }

    #[test]
    fn interval_test_empty_test() {
        let data = [100; 0];
        let cum = CumSum::new(&data);
        assert_eq!(cum.interval_sum(..), 0);
        assert_eq!(cum.interval_sum(0..1), 0);
        assert_eq!(cum.interval_sum(0..100), 0);
    }
}
