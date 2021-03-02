//   data index:  0 1 2 3 4
// cumsum index: 0 1 2 3 4 5

use num::Zero;
use std::{
    iter::FromIterator,
    ops::{Add, Bound, RangeBounds, Sub},
};

pub struct CumSum<T> {
    v: Vec<T>,
}
impl<T: Clone + Add<Output = T> + Sub<Output = T> + Zero> FromIterator<T> for CumSum<T> {
    /// **O(n)**, create n+1 size vec for cumsum from iterator
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let v = vec![T::zero()]
            .into_iter()
            .chain(iter.into_iter())
            .scan(T::zero(), |cum, x| {
                *cum = cum.clone() + x;
                Some(cum.clone())
            })
            .collect();
        CumSum { v }
    }
}
impl<'a, T: 'a + Clone + Add<Output = T> + Sub<Output = T> + Zero> FromIterator<&'a T>
    for CumSum<T>
{
    /// **O(n)**, create n+1 size vec for cumsum from borrowed iterator
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        iter.into_iter().cloned().collect()
    }
}
impl<T: Clone + Add<Output = T> + Sub<Output = T> + Zero> CumSum<T> {
    /// **O(n)**, create n+1 size vec for cumsum from borrowed iterator
    pub fn with_data<I, Ii>(data: I) -> Self
    where
        I: IntoIterator<Item = Ii>,
        Self: FromIterator<Ii>,
    {
        data.into_iter().collect()
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
        self.v[right].clone() - self.v[left].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cumsum_collect_interface_test() {
        // from array, use iter() method
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum: CumSum<_> = data.iter().collect();
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is not moved

        // from vec, use iter() method
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum: CumSum<_> = data.iter().collect();
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is not moved

        // from vec, use into_iter() method (data is moved)
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum: CumSum<_> = data.into_iter().collect();
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        // assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is moved!

        // from range (data is moved)
        let data = 1..=10;
        let cum: CumSum<_> = data.into_iter().collect();
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        // assert_eq!(data, 1..=10); // data is moved!
    }

    #[test]
    fn cumsum_with_data_interface_test() {
        // from array reference
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::with_data(&data);
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is not moved

        // from vec, use iter() method
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::with_data(data.iter());
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is not moved

        // from vec (data is moved)
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::with_data(data);
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        // assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // data is moved!

        // from range (data is moved)
        let data = 1..=10;
        let cum = CumSum::with_data(data);
        assert_eq!(cum.v, vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
        // assert_eq!(data, 1..=10); // data is moved!
    }

    #[test]
    fn interval_sum_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::with_data(data);
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
        let cum: CumSum<_> = (0..=10).collect();
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
        let cum = CumSum::with_data(&data);
        assert_eq!(cum.interval_sum(..), 0);
        assert_eq!(cum.interval_sum(0..1), 0);
        assert_eq!(cum.interval_sum(0..100), 0);
    }
}
