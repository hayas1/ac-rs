use std::ops::AddAssign;

use num::Integer;

/// **O(log(end-start))**, return (start, end) that f(end) == true (should f(start) == false)
pub fn initial_indices<I, F>(start: I, end: Option<I>, f: &F) -> (I, I)
where
    I: Clone + Integer + AddAssign,
    F: Fn(&I) -> bool,
{
    if let Some(end) = end {
        (start, end)
    } else {
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
}

/// **O(log(end-start))**, find the first index at which false -> true (f(start) must be false)
pub fn bisect<I, F>(start: I, end: Option<I>, f: F) -> I
where
    I: Clone + Integer + AddAssign,
    F: Fn(&I) -> bool,
{
    let (mut left, mut right) = initial_indices(start, end, &f);
    while left.clone() + I::one() < right {
        let mid = (left.clone() + right.clone()) / (I::one() + I::one());
        if f(&mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bisect_test() {
        let a6 = [false, false, false, true, true, true];
        assert_eq!(bisect(0, Some(a6.len()), |&i| a6[i]), 3);
        assert_eq!(bisect(0, None, |&i| a6[i]), 3);
        let a7false = [false, false, false, false, true, true, true];
        assert_eq!(bisect(0, Some(a7false.len()), |&i| a7false[i]), 4);
        assert_eq!(bisect(0, None, |&i| a7false[i]), 4);
        let a7true = [false, false, false, true, true, true, true];
        assert_eq!(bisect(0, Some(a7true.len()), |&i| a7true[i]), 3);
        assert_eq!(bisect(0, None, |&i| a7true[i]), 3);
        let a8 = [false, false, false, false, true, true, true, true];
        assert_eq!(bisect(0, Some(a8.len()), |&i| a8[i]), 4);
        assert_eq!(bisect(0, None, |&i| a8[i]), 4);
    }
}
