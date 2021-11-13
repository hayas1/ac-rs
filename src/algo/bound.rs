/// **O(log(|a|))**, return the first index i such that a[i] >= x
pub fn upper_bound<T: PartialOrd>(a: &[T], x: T) -> usize {
    let (mut start, mut size) = (0, a.len());
    while size > 1 {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn upper_bound_test() {
        let v = vec![1, 2, 4, 5, 7, 8, 10];
        assert_eq!(upper_bound(&v, 0), 0);
        assert_eq!(upper_bound(&v, 1), 0);
        assert_eq!(upper_bound(&v, 3), 2);
        assert_eq!(upper_bound(&v, 4), 2);
        assert_eq!(upper_bound(&v, 9), 6);
        assert_eq!(upper_bound(&v, 10), 6);
        assert_eq!(upper_bound(&v, 100), 6);
    }

    #[test]
    fn upper_bound_bound_test() {
        let v = vec![10; 0];
        assert_eq!(upper_bound(&v, 10), 0);
        let v = vec![10; 1];
        assert_eq!(upper_bound(&v, 1), 0);
        assert_eq!(upper_bound(&v, 9), 0);
        assert_eq!(upper_bound(&v, 10), 0);
        assert_eq!(upper_bound(&v, 11), 0);
    }
}
