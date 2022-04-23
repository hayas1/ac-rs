/// **O(n)**, get hamming distance between a and b such as a.len() == len()
pub fn hamming_distance<T: Eq, I: IntoIterator<Item = T>>(a: I, b: I) -> usize {
    a.into_iter().zip(b.into_iter()).filter(|(ai, bi)| ai != bi).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming1() {
        let a: Vec<_> = "rust".chars().collect();
        let b: Vec<_> = "ruby".chars().collect();
        assert_eq!(hamming_distance(&a, &a), 0);
        assert_eq!(hamming_distance(&a, &b), 2);
    }

    #[test]
    fn test_hamming2() {
        let a = [1, 2, 1, 3];
        let b = [1, 3, 1, 2];
        assert_eq!(hamming_distance(&a, &b), 2);
    }

    #[test]
    fn test_hamming3() {
        let a = [1, 3, 2, 4, 6, 9];
        let b = [1, 5, 2, 6, 4, 3];
        assert_eq!(hamming_distance(&a, &b), 4);
    }

    #[test]
    fn test_hamming4() {
        let a = vec![1, 1, 1, 1, 1];
        let b = vec![1, 1, 1, 1, 1];
        assert_eq!(hamming_distance(a, b), 0);
    }

    #[test]
    fn test_hamming_bound() {
        assert_eq!(hamming_distance("".chars(), "".chars()), 0);
        assert_eq!(hamming_distance(&[0; 0], &[0; 0]), 0);
    }
}
