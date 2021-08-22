/// **O(n)**, generate next item in dictionary order, in place
pub fn next_permutation<T: Ord>(s: &mut [T]) -> bool {
    if let Some(i) = s.windows(2).rposition(|w| w[0] < w[1]) {
        let j = s.iter().rposition(|b| &s[i] < b).expect("exist i + 1");
        s.swap(i, j);
        s[i + 1..].reverse();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_permutation_test1() {
        let mut s: Vec<_> = "aabbcc".chars().collect();
        assert!(next_permutation(&mut s));
        assert_eq!("aabcbc".chars().collect::<Vec<_>>(), s);
        assert!(next_permutation(&mut s));
        assert_eq!("aabccb".chars().collect::<Vec<_>>(), s);
        assert!(next_permutation(&mut s));
        assert_eq!("aacbbc".chars().collect::<Vec<_>>(), s);
        assert!(next_permutation(&mut s));
        assert_eq!("aacbcb".chars().collect::<Vec<_>>(), s);
        assert!(next_permutation(&mut s));
        assert_eq!("aaccbb".chars().collect::<Vec<_>>(), s);
        assert!(next_permutation(&mut s));
        assert_eq!("ababcc".chars().collect::<Vec<_>>(), s);
    }

    #[test]
    fn next_permutation_test2() {
        let mut s: Vec<_> = "acb".chars().collect();
        assert_eq!(true, next_permutation(&mut s));
        assert_eq!("bac".chars().collect::<Vec<_>>(), s);
        assert_eq!(true, next_permutation(&mut s));
        assert_eq!("bca".chars().collect::<Vec<_>>(), s);
        assert_eq!(true, next_permutation(&mut s));
        assert_eq!("cab".chars().collect::<Vec<_>>(), s);
        assert_eq!(true, next_permutation(&mut s));
        assert_eq!("cba".chars().collect::<Vec<_>>(), s);
        assert_eq!(false, next_permutation(&mut s));
        assert_eq!("cba".chars().collect::<Vec<_>>(), s);
    }
}
