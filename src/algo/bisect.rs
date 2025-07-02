use std::ops::{AddAssign, Bound, RangeBounds, SubAssign};

use num::{traits::Pow, Integer, Num};

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
        Bound::Excluded(start) => start.clone() + T::one(),
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
/// **O(log(ans/unit))**, find the first index (width: `unit`) at which false -> true (f(start) must be false)
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

/// **O(log(x))**, calculate square root of x by ceil (warning: overflow)
pub fn sqrt_ceil<T: Clone + PartialOrd + AddAssign + SubAssign + Integer>(x: T) -> T {
    bisect(T::one()..=x.clone(), |i| i.clone() * i.clone() >= x).unwrap_or(x)
}
/// **O(log(x))**, calculate square root of x by floor (warning: overflow)
pub fn sqrt_floor<T: Clone + PartialOrd + AddAssign + SubAssign + Integer>(x: T) -> T {
    bisect(T::one()..=x.clone(), |i| i.clone() * i.clone() > x).unwrap_or(x + T::one()) - T::one()
}
/// **O(log())**, calculate log_a(x) by ceil (warning: overflow)
pub fn log_ceil<T, P>(a: T, x: P) -> P
where
    P: Clone + PartialOrd + AddAssign + SubAssign + Integer,
    T: Clone + PartialOrd + From<P> + Pow<P, Output = T>,
{
    bisect(P::one()..x.clone(), |i| a.clone().pow(i.clone()) >= x.clone().into()).unwrap_or_else(
        || {
            if a > x.into() {
                P::zero()
            } else {
                P::one()
            }
        },
    )
}
/// **O(log())**, calculate log_a(x) by floor (warning: overflow)
pub fn log_floor<T, P>(a: T, x: P) -> P
where
    P: Clone + PartialOrd + AddAssign + SubAssign + Integer,
    T: Clone + PartialOrd + From<P> + Pow<P, Output = T>,
{
    bisect(P::one()..x.clone(), |i| a.clone().pow(i.clone()) > x.clone().into()).unwrap_or_else(
        || {
            if a > x.into() {
                P::one()
            } else {
                P::one() + P::one()
            }
        },
    ) - P::one()
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
    fn test_bisect_normal() {
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
    fn test_bisect_integer() {
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
    fn test_bisect_float() {
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
    fn test_arith_sqrt_ceil() {
        assert_eq!(sqrt_ceil(0), 0);
        assert_eq!(sqrt_ceil(1), 1);
        assert_eq!(sqrt_ceil(2), 2);
        assert_eq!(sqrt_ceil(3), 2);
        assert_eq!(sqrt_ceil(4), 2);
        assert_eq!(sqrt_ceil(5), 3);
        assert_eq!(sqrt_ceil(10), 4);
        assert_eq!(sqrt_ceil(99), 10);
        assert_eq!(sqrt_ceil(100), 10);
        assert_eq!(sqrt_ceil(101), 11);
    }

    #[test]
    fn test_arith_sqrt_floor() {
        assert_eq!(sqrt_floor(0), 0);
        assert_eq!(sqrt_floor(1), 1);
        assert_eq!(sqrt_floor(2), 1);
        assert_eq!(sqrt_floor(3), 1);
        assert_eq!(sqrt_floor(4), 2);
        assert_eq!(sqrt_floor(5), 2);
        assert_eq!(sqrt_floor(10), 3);
        assert_eq!(sqrt_floor(99), 9);
        assert_eq!(sqrt_floor(100), 10);
        assert_eq!(sqrt_floor(101), 10);
    }

    #[test]
    fn test_arith_log_ceil() {
        assert_eq!(log_ceil(2u64, 0u32), 0); // 2^0 != 0, but...
        assert_eq!(log_ceil(2u64, 1u32), 0);
        assert_eq!(log_ceil(2u64, 2u32), 1);
        assert_eq!(log_ceil(2u64, 3u32), 2);
        assert_eq!(log_ceil(2u64, 4u32), 2);
        assert_eq!(log_ceil(2u64, 5u32), 3);
        assert_eq!(log_ceil(2u64, 7u32), 3);
        assert_eq!(log_ceil(2u64, 8u32), 3);
        assert_eq!(log_ceil(2u64, 9u32), 4);
        assert_eq!(log_ceil(10u64, 9u32), 0);
        assert_eq!(log_ceil(10u64, 10u32), 1);
        assert_eq!(log_ceil(10u64, 11u32), 2);
    }

    #[test]
    fn test_arith_log_floor() {
        assert_eq!(log_floor(2u64, 0u32), 0); // 2^0 != 0, but...
        assert_eq!(log_floor(2u64, 1u32), 0);
        assert_eq!(log_floor(2u64, 2u32), 1);
        assert_eq!(log_floor(2u64, 3u32), 1);
        assert_eq!(log_floor(2u64, 4u32), 2);
        assert_eq!(log_floor(2u64, 5u32), 2);
        assert_eq!(log_floor(2u64, 7u32), 2);
        assert_eq!(log_floor(2u64, 8u32), 3);
        assert_eq!(log_floor(2u64, 9u32), 3);
        assert_eq!(log_floor(10u64, 9u32), 0);
        assert_eq!(log_floor(10u64, 10u32), 1);
        assert_eq!(log_floor(10u64, 11u32), 1);
    }

    #[test]
    fn test_bisect_left() {
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
    fn test_bisect_right() {
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
    fn test_empty_bisect() {
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
