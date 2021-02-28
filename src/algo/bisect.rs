use std::ops::{AddAssign, Bound, RangeBounds, SubAssign};

use num::{Integer, Num};

/// **O(log(end-start))**, return (start, end) that f(end) == true (should f(start) == false)
pub fn initial_indices<T, R, F>(range: R, f: &F) -> (T, T)
where
    T: Clone + PartialOrd + AddAssign + SubAssign + Num,
    R: RangeBounds<T>,
    F: Fn(&T) -> bool,
{
    let start = match range.start_bound() {
        Bound::Unbounded => {
            let (mut guessed_start, mut range) = (T::zero(), T::one());
            while f(&guessed_start) {
                // if f has no proper property, it will cause infinite loop
                range += range.clone();
                guessed_start -= range.clone();
            }
            guessed_start
        }
        Bound::Excluded(start) => (start.clone() + T::one()),
        Bound::Included(start) => start.clone(),
    };
    let (start, end) = match range.end_bound() {
        Bound::Unbounded => {
            let (mut guessed_start, mut range) = (start.clone(), T::one());
            let guessed_end = loop {
                // if f has no proper property, it will cause infinite loop
                let end = start.clone() + range.clone();
                if f(&end) {
                    break end;
                } else {
                    guessed_start = end;
                    range += range.clone();
                }
            };
            (guessed_start, guessed_end)
        }
        Bound::Excluded(end) => {
            (start, if end > &T::one() { end.clone() - T::one() } else { T::zero() })
        }
        Bound::Included(end) => (start, end.clone()),
    };
    (start, end)
}
/// **O(log(ans))**, find the first index at which false -> true (f(start) must be false)
pub fn bisect_unit<T, R, F>(range: R, unit: T, f: F) -> Option<T>
where
    T: Clone + PartialOrd + AddAssign + SubAssign + Num,
    R: RangeBounds<T>,
    F: Fn(&T) -> bool,
{
    let (mut start, mut end) = initial_indices(range, &f);
    if start >= end || f(&start) || !f(&end) {
        return None; // if f(start) == true then all is true, and if f(end)==false then all is false
    }
    while end.clone() - start.clone() > unit {
        let mid = (start.clone() + end.clone()) / (T::one() + T::one());
        if f(&mid) {
            end = mid;
        } else {
            start = mid;
        }
    }
    Some(end)
}
/// **O(log(ans))**, find the first index at which false -> true (f(start) must be false)
pub fn bisect<T, R, F>(range: R, f: F) -> Option<T>
where
    T: Clone + PartialOrd + AddAssign + SubAssign + Integer,
    R: RangeBounds<T>,
    F: Fn(&T) -> bool,
{
    bisect_unit(range, T::one(), f)
}

/// **O(log(n))**, find the leftmost insertion index with key function
pub fn bisect_left_by_key<T, U, K>(a: &[T], x: U, k: K) -> usize
where
    U: PartialOrd,
    K: Fn(&T) -> &U,
{
    if let Some(i) =
        bisect(0..a.len(), |&i| a.get(i).and_then(|xs| Some(k(xs) >= &x)).unwrap_or(false))
    {
        i
    } else {
        if let Some(last) = a.last() {
            if k(last) < &x {
                return a.len();
            }
        }
        0
    }
}
/// **O(log(n))**, find the leftmost insertion index with key function
pub fn bisect_left<T: PartialOrd>(a: &[T], x: T) -> usize {
    bisect_left_by_key(&a, x, |k| k)
}

/// **O(log(n))**, find the rightmost insertion index with key function
pub fn bisect_right_by_key<T, U, K>(a: &[T], x: U, k: K) -> usize
where
    U: PartialOrd,
    K: Fn(&T) -> &U,
{
    if let Some(i) =
        bisect(0..a.len(), |&i| a.get(i).and_then(|xs| Some(k(xs) > &x)).unwrap_or(false))
    {
        i
    } else {
        if let Some(last) = a.last() {
            if k(last) <= &x {
                return a.len();
            }
        }
        0
    }
}
/// **O(log(n))**, find the rightmost insertion index with key function
pub fn bisect_right<T: PartialOrd>(a: &[T], x: T) -> usize {
    bisect_right_by_key(&a, x, |k| k)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bisect_normal_test() {
        let a6 = [false, false, false, true, true, true];
        assert_eq!(bisect(0..a6.len(), |&i| a6[i]), Some(3));
        assert_eq!(bisect(0.., |&i| a6[i]), Some(3));
        let a7false = [false, false, false, false, true, true, true];
        assert_eq!(bisect(0..a7false.len(), |&i| a7false[i]), Some(4));
        assert_eq!(bisect(0.., |&i| a7false[i]), Some(4));
        let a7true = [false, false, false, true, true, true, true];
        assert_eq!(bisect(0..a7true.len(), |&i| a7true[i]), Some(3));
        assert_eq!(bisect(0.., |&i| a7true[i]), Some(3));
        let a8 = [false, false, false, false, true, true, true, true];
        assert_eq!(bisect(0..a8.len(), |&i| a8[i]), Some(4));
        assert_eq!(bisect(0.., |&i| a8[i]), Some(4));
    }

    #[test]
    fn bisect_integer_test() {
        let x_pow_2 = |x| x * x;
        assert_eq!(bisect(0.., |&i| x_pow_2(i) > 100), Some(11));
        assert_eq!(bisect(..11, |&i| x_pow_2(i) > 100), None);
        assert_eq!(bisect(..=11, |&i| x_pow_2(i) > 100), Some(11));
        assert_eq!(bisect(..=10, |&i| x_pow_2(i) > 100), None);
        assert_eq!(bisect(..=10, |&i| x_pow_2(i) >= 100), Some(10));
        let x_pow_3 = |x| x * x * x;
        assert_eq!(bisect(.., |&i| x_pow_3(i) > 100), Some(5));
        assert_eq!(bisect(.., |&i| x_pow_3(i) > -100), Some(-4));
    }

    #[test]
    fn bisect_float_test() {
        let x_pow_2 = |x| x * x;
        assert!((10.0..10.05).contains(&bisect_unit(0.0.., 0.05, |&i| x_pow_2(i) > 100.).unwrap()));
        assert_eq!(bisect_unit(..11., 0.05, |&i| x_pow_2(i) > 100.), None);
        assert!((10.0..10.05).contains(&bisect_unit(..=11., 0.05, |&i| x_pow_2(i) > 100.).unwrap()));
        assert_eq!(bisect_unit(..=10., 0.05, |&i| x_pow_2(i) > 100.), None);
        assert!((10.0..10.05).contains(&bisect_unit(..=11., 0.05, |&i| x_pow_2(i) > 100.).unwrap()));
        assert!((31.622776..31.622777) // sqrt(1000)
            .contains(&bisect_unit(.., 0.00000001, |&i| x_pow_2(i) > 1000.).unwrap()));
    }

    #[test]
    fn bisect_left_test() {
        let a = [1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(bisect_left(&a, 0), 0);
        assert_eq!(bisect_left(&a, 1), 0);
        assert_eq!(bisect_left(&a, 2), 2);
        assert_eq!(bisect_left(&a, 4), 4);
        assert_eq!(bisect_left(&a, 21), 7);
        assert_eq!(bisect_left(&a, 34), 8);
        assert_eq!(bisect_left(&a, 35), 9);
        let b = [1., 1., 1.141, 1.732, 2., 2.236];
        assert_eq!(bisect_left(&b, 1.), 0);
        assert_eq!(bisect_left(&b, 1.5), 3);
        assert_eq!(bisect_left(&b, 0.), 0);
        assert_eq!(bisect_left(&b, 3.14), 6);
        let c = [(1, "one1"), (1, "one2"), (3, "three"), (5, "five"), (6, "six")];
        assert_eq!(bisect_left_by_key(&c, 0, |k| &k.0), 0);
        assert_eq!(bisect_left_by_key(&c, 1, |k| &k.0), 0);
        assert_eq!(bisect_left_by_key(&c, 2, |k| &k.0), 2);
        assert_eq!(bisect_left_by_key(&c, 3, |k| &k.0), 2);
        assert_eq!(bisect_left_by_key(&c, 6, |k| &k.0), 4);
        assert_eq!(bisect_left_by_key(&c, 10000, |k| &k.0), 5);
    }

    #[test]
    fn bisect_right_test() {
        let a = [1, 1, 2, 3, 5, 8, 13, 21, 34];
        assert_eq!(bisect_right(&a, 0), 0);
        assert_eq!(bisect_right(&a, 1), 2);
        assert_eq!(bisect_right(&a, 2), 3);
        assert_eq!(bisect_right(&a, 4), 4);
        assert_eq!(bisect_right(&a, 21), 8);
        assert_eq!(bisect_right(&a, 34), 9);
        assert_eq!(bisect_right(&a, 35), 9);
        let b = [1., 1., 1.141, 1.732, 2., 2.236];
        assert_eq!(bisect_right(&b, 1.), 2);
        assert_eq!(bisect_right(&b, 1.5), 3);
        assert_eq!(bisect_right(&b, 0.), 0);
        assert_eq!(bisect_right(&b, 3.14), 6);
        let c = [(1, "one1"), (1, "one2"), (3, "three"), (5, "five"), (6, "six")];
        assert_eq!(bisect_right_by_key(&c, 0, |k| &k.0), 0);
        assert_eq!(bisect_right_by_key(&c, 1, |k| &k.0), 2);
        assert_eq!(bisect_right_by_key(&c, 2, |k| &k.0), 2);
        assert_eq!(bisect_right_by_key(&c, 3, |k| &k.0), 3);
        assert_eq!(bisect_right_by_key(&c, 5, |k| &k.0), 4);
        assert_eq!(bisect_right_by_key(&c, 6, |k| &k.0), 5);
        assert_eq!(bisect_right_by_key(&c, 10000, |k| &k.0), 5);
    }

    #[test]
    fn empty_bisect_test() {
        let x_pow_2 = |x| x * x;
        // by definition, range size < 2 then return None
        assert_eq!(bisect(10..11, |&i| x_pow_2(i) >= 100), None);
        assert_eq!(bisect(10..10, |&i| x_pow_2(i) >= 100), None);
        let e1 = [2; 0];
        assert_eq!(bisect_left(&e1, 10), 0);
        assert_eq!(bisect_right(&e1, 10), 0);
        let e2 = [(2, "two"); 0];
        assert_eq!(bisect_left_by_key(&e2, 10, |k| &k.0), 0);
        assert_eq!(bisect_right_by_key(&e2, 10, |k| &k.0), 0);
    }
}
