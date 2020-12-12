#![allow(dead_code)]
//   data index:  0 1 2 3 4
// cumsum index: 0 1 2 3 4 5

use num::Zero;
use std::ops::{Add, Sub};

struct CumSum<T> {
    v: Vec<T>,
}
impl<T: Add<Output = T> + Sub<Output = T> + Zero + Copy> CumSum<T> {
    /// **O(n)** create n+1 size vec for cumsum
    fn new(data: &[T]) -> Self {
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

    /// **O(1)** calculate half-open interval summation [l, r)
    fn interval_sum(&self, l: usize, r: usize) -> T {
        self.v[r] - self.v[l]
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
        assert_eq!(cum.interval_sum(0, 3), 3);
        assert_eq!(cum.interval_sum(3, 6), 12);
        assert_eq!(cum.interval_sum(6, 9), 21);
    }

    #[test]
    fn interval_sum_bound_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let cum = CumSum::new(&data);
        assert_eq!(cum.interval_sum(0, 0), 0);
        assert_eq!(cum.interval_sum(1, 2), 1);
        assert_eq!(cum.interval_sum(6, 9), 21);
        assert_eq!(cum.interval_sum(1, 10), 45);
        assert_eq!(cum.interval_sum(0, 11), 55);
    }
}
