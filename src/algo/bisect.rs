use std::ops::{AddAssign, Bound, RangeBounds};

use num::Num;

/// **O(log(end-start))**, return (start, end) that f(end) == true (should f(start) == false)
pub fn initial_indices<I, R, F>(range: R, f: &F) -> (I, I)
where
    I: Clone + PartialOrd + AddAssign + Num,
    R: RangeBounds<I>,
    F: Fn(&I) -> bool,
{
    let start = match range.start_bound() {
        Bound::Unbounded => I::zero(),
        Bound::Excluded(start) => (start.clone() + I::one()),
        Bound::Included(start) => start.clone(),
    };
    let (start, end) = match range.end_bound() {
        Bound::Unbounded => {
            let (mut guessed_start, mut range) = (start.clone(), I::one());
            let guessed_end = loop {
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
            (start, if end > &I::one() { end.clone() - I::one() } else { I::zero() })
        }
        Bound::Included(end) => (start, end.clone()),
    };
    assert!(start <= end);
    (start, end)
}

/// **O(log(ans))**, find the first index at which false -> true (f(start) must be false)
pub fn bisect<I, R, F>(range: R, f: F) -> Option<I>
where
    I: Clone + PartialOrd + AddAssign + Num,
    R: RangeBounds<I>,
    F: Fn(&I) -> bool,
{
    let (mut start, mut end) = initial_indices(range, &f);
    if f(&start) || !f(&end) {
        return None; // if f(start) == true then all is true, and if f(end)==false then all is false
    }
    while start.clone() + I::one() < end {
        let mid = (start.clone() + end.clone()) / (I::one() + I::one());
        if f(&mid) {
            end = mid;
        } else {
            start = mid;
        }
    }
    Some(end)
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
    }

    #[test]
    fn bisect_float_test() {
        // warning: bisect is discrete(`while start + I::one() < end`), so float is deprecated
        let x_pow_2 = |x| x * x;
        assert_eq!(bisect(0.0.., |&i| x_pow_2(i) > 100.), Some(11.));
        assert_eq!(bisect(..11., |&i| x_pow_2(i) > 100.), None);
        assert_eq!(bisect(..=11., |&i| x_pow_2(i) > 100.), Some(10.3125));
        assert_eq!(bisect(..=10., |&i| x_pow_2(i) > 100.), None);
        assert_eq!(bisect(..=10., |&i| x_pow_2(i) >= 100.), Some(10.));
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
}
