//   data index:  0 1 2 3 4
// imos1d index: 0 1 2 3 4 5

use num::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct Imos1D<T> {
    v: Vec<T>,
}
impl<T> Imos1D<T>
where
    T: Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + Zero + Copy,
{
    /// **O(n)** create n+1 size vec for imos 1d
    pub fn new(data: &[(usize, usize, T)], max: Option<usize>) -> Self {
        let mut imos = match max {
            Some(m) => vec![T::zero(); m + 1],
            None => vec![
                T::zero();
                1 + data
                    .iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap_or(&(0, 0, T::zero()))
                    .1
            ],
        };
        for &(s, t, u) in data.iter() {
            imos[s] += u;
            imos[t] -= u;
        }
        let v = imos
            .iter()
            .scan(T::zero(), |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect();
        Imos1D { v }
    }

    /// **O(1)** calculate sum of timing t
    pub fn sum_timing(&self, t: usize) -> T {
        self.v[t]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imos1d_test() {
        // time index:    0 1 2 3 4 5 6
        // imos1d index: 0 1 2 3 4 5 6 7
        // a 1-2 val 3:     3
        // b 1-5 val 2:     2 2 2 2
        // c 3-6 val 1:         1 1 1
        // imos1d array:  0 5 2 3 3 1 0
        let data = vec![(1, 2, 3), (1, 5, 2), (3, 6, 1)];
        let imos = Imos1D::new(&data, None);
        assert_eq!(imos.v, vec![0, 5, 2, 3, 3, 1, 0]);
        assert_eq!(imos.sum_timing(3), 3);
    }

    #[test]
    fn imos1d_size_test() {
        let data = vec![(1, 2, 3), (1, 5, 2), (3, 6, 1)];
        let imos = Imos1D::new(&data, Some(10));
        assert_eq!(imos.v, vec![0, 5, 2, 3, 3, 1, 0, 0, 0, 0, 0]);
        assert_eq!(imos.sum_timing(3), 3);
    }

    #[test]
    fn imos1d_empty_test() {
        let data = Vec::new();
        let imos = Imos1D::<isize>::new(&data, None);
        assert_eq!(imos.v, vec![0]);
        assert_eq!(imos.sum_timing(0), 0);
    }
}
