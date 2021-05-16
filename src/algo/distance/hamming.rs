/// **O(n)**, get hamming distance between a and b such as a.len() == len()
pub fn hamming_distance<T: Eq, I: IntoIterator<Item = T>>(a: I, b: I) -> usize {
    a.into_iter().zip(b.into_iter()).filter(|(ai, bi)| ai != bi).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_test1() {
        let a: Vec<_> = "xy".chars().collect();
        let b: Vec<_> = "xy".chars().collect();
        assert_eq!(hamming_distance(&a, &b), 0);
    }

    #[test]
    fn hamming_test2() {
        let a = [1, 2, 1, 3];
        let b = [1, 3, 1, 2];
        assert_eq!(hamming_distance(&a, &b), 2);
    }

    #[test]
    fn hamming_test3() {
        let a = [1, 3, 2, 4, 6, 9];
        let b = [1, 5, 2, 6, 4, 3];
        assert_eq!(hamming_distance(&a, &b), 4);
    }

    #[test]
    fn hamming_test4() {
        let a = vec![1, 1, 1, 1, 1];
        let b = vec![1, 1, 1, 1, 1];
        assert_eq!(hamming_distance(a, b), 0);
    }

    #[test]
    fn hamming_bound_test() {
        assert_eq!(hamming_distance("rust".chars(), "ruby".chars()), 2);
        assert_eq!(hamming_distance(&[0; 0], &[0; 0]), 0);
    }
}
