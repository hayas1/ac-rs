/// **O(log(|a|))**, return the first index i such that a[i] > x
pub fn upper_bound<T: PartialOrd>(a: &[T], x: T) -> usize {
    let (mut start, mut size) = (0, a.len());
    while size > 0 {
        let half = size / 2;
        let mid = start + half;
        if x < a[mid] {
            size = half;
        } else {
            size -= half + 1;
            start = mid + 1;
        }
    }
    start
}

/// **O(log(|a|))**, return the first index i such that a[i] >= x
pub fn lower_bound<T: PartialOrd>(a: &[T], x: T) -> usize {
    let (mut start, mut size) = (0, a.len());
    while size > 0 {
        let half = size / 2;
        let mid = start + half;
        if a[mid] < x {
            size -= half + 1;
            start = mid + 1;
        } else {
            size = half;
        }
    }
    start
}

/// **O(log(|a|))**, return the last index i such that a[i] <= x
pub fn bisect_right<T: PartialOrd>(a: &[T], x: T) -> usize {
    let (mut start, mut end) = (0, a.len());
    while start < end {
        let mid = (start + end) / 2;
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
    fn test_upper_bound() {
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
    fn test_upper_bound_bound() {
        let v = vec![10; 0];
        assert_eq!(upper_bound(&v, 10), 0);
        let v = vec![10; 1];
        assert_eq!(upper_bound(&v, 1), 0);
        assert_eq!(upper_bound(&v, 9), 0);
        assert_eq!(upper_bound(&v, 10), 1);
        assert_eq!(upper_bound(&v, 11), 1);
    }

    #[test]
    fn test_lower_bound() {
        let v = vec![1, 2, 4, 5, 7, 8, 10];
        assert_eq!(lower_bound(&v, 0), 0);
        assert_eq!(lower_bound(&v, 1), 0);
        assert_eq!(lower_bound(&v, 3), 2);
        assert_eq!(lower_bound(&v, 4), 2);
        assert_eq!(lower_bound(&v, 9), 6);
        assert_eq!(lower_bound(&v, 10), 6);
        assert_eq!(lower_bound(&v, 100), 7);
    }

    #[test]
    fn test_lower_bound_bound() {
        let v = vec![10; 0];
        assert_eq!(lower_bound(&v, 10), 0);
        let v = vec![10; 1];
        assert_eq!(lower_bound(&v, 1), 0);
        assert_eq!(lower_bound(&v, 9), 0);
        assert_eq!(lower_bound(&v, 10), 0);
        assert_eq!(lower_bound(&v, 11), 1);
    }

    #[test]
    fn test_same_element() {
        let v = vec![3, 3, 3, 4, 4, 4, 4, 7, 7, 9, 10];
        assert_eq!(lower_bound(&v, 2), 0);
        assert_eq!(upper_bound(&v, 2), 0);
        assert_eq!(lower_bound(&v, 3), 0);
        assert_eq!(upper_bound(&v, 3), 3);
        assert_eq!(lower_bound(&v, 4), 3);
        assert_eq!(upper_bound(&v, 4), 7);
        assert_eq!(lower_bound(&v, 5), 7);
        assert_eq!(upper_bound(&v, 5), 7);
        assert_eq!(lower_bound(&v, 6), 7);
        assert_eq!(upper_bound(&v, 6), 7);
        assert_eq!(lower_bound(&v, 7), 7);
        assert_eq!(upper_bound(&v, 7), 9);
        assert_eq!(lower_bound(&v, 8), 9);
        assert_eq!(upper_bound(&v, 8), 9);
        assert_eq!(lower_bound(&v, 9), 9);
        assert_eq!(upper_bound(&v, 9), 10);
        assert_eq!(lower_bound(&v, 10), 10);
        assert_eq!(upper_bound(&v, 10), 11);
        assert_eq!(lower_bound(&v, 11), 11);
        assert_eq!(upper_bound(&v, 11), 11);
    }

    #[test]
    fn test_bisect_right() {
        let v = vec![3, 3, 3, 4, 4, 4, 4, 7, 7, 9, 10];
        assert_eq!(bisect_right(&v, 2), 0);
        assert_eq!(bisect_right(&v, 3), 3);
        assert_eq!(bisect_right(&v, 4), 7);
        assert_eq!(bisect_right(&v, 5), 7);
        assert_eq!(bisect_right(&v, 6), 7);
        assert_eq!(bisect_right(&v, 7), 9);
        assert_eq!(bisect_right(&v, 8), 9);
        assert_eq!(bisect_right(&v, 9), 10);
        assert_eq!(bisect_right(&v, 10), 11);
        assert_eq!(bisect_right(&v, 11), 11);
    }
}
