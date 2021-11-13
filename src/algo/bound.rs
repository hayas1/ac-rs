use num::Integer;

/// **O(log(|a|))**, return first index such as a[i] < x
pub fn upper_bound<T: Clone + PartialOrd + Integer>(a: &[T], x: T) -> usize {
    let (mut start, mut end) = (0, a.len());
    while start < end {
        let mid = (start.clone() + end.clone()) / 2;
        if x < a[mid] {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    start
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn upper_bound_test() {
        let v = vec![1, 2, 4, 5, 7, 8, 10];
        assert_eq!(upper_bound(&v, 0), 0);
        assert_eq!(upper_bound(&v, 1), 1);
        assert_eq!(upper_bound(&v, 3), 2);
        assert_eq!(upper_bound(&v, 4), 3);
        assert_eq!(upper_bound(&v, 9), 6);
        assert_eq!(upper_bound(&v, 10), 7);
        assert_eq!(upper_bound(&v, 100), 7);
    }

    #[test]
    fn upper_bound_bound_test() {
        let v = vec![10; 0];
        assert_eq!(upper_bound(&v, 10), 0);
        let v = vec![10; 1];
        assert_eq!(upper_bound(&v, 1), 0);
        assert_eq!(upper_bound(&v, 9), 0);
        assert_eq!(upper_bound(&v, 10), 1);
        assert_eq!(upper_bound(&v, 11), 1);
    }
}
