#![allow(dead_code)]

/// **O(nm)** calculate edit distance between sequence a and sequence b
pub fn levenshtein_distance<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let (n, m) = (a.len(), b.len());
    if n == 0 || m == 0 {
        return std::cmp::max(n, m);
    }
    // dp[i][j]: edit distance between a[0..=i] and b[0..=j]
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = if a[0] == b[0] { 0 } else { 1 };
    for i in 1..n {
        let condition = dp[i - 1][0] == i && a[i] == b[0];
        dp[i][0] = dp[i - 1][0] + if condition { 0 } else { 1 };
    }
    for j in 1..m {
        let condition = dp[0][j - 1] == j && a[0] == b[j];
        dp[0][j] = dp[0][j - 1] + if condition { 0 } else { 1 };
    }
    for i in 1..n {
        for j in 1..m {
            let add_a = dp[i - 1][j] + 1;
            let add_b = dp[i][j - 1] + 1;
            let modify = dp[i - 1][j - 1] + if a[i] == b[j] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(add_a, add_b).min(modify);
        }
    }
    dp[n - 1][m - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_test1() {
        let a: Vec<_> = "xxy".chars().collect();
        let b: Vec<_> = "xy".chars().collect();
        assert_eq!(levenshtein_distance(&a, &b), 1);
    }

    #[test]
    fn levenshtein_test2() {
        let a = [1, 2, 1, 3];
        let b = [1, 3, 1];
        assert_eq!(levenshtein_distance(&a, &b), 2);
    }

    #[test]
    fn levenshtein_test3() {
        let a = [1, 3, 2, 4];
        let b = [1, 5, 2, 6, 4, 3];
        assert_eq!(levenshtein_distance(&a, &b), 3);
    }

    #[test]
    fn levenshtein_test4() {
        let a = vec![1, 1, 1, 1, 1];
        let b = vec![2, 2, 2, 2, 2];
        assert_eq!(levenshtein_distance(&a, &b), 5);
    }

    #[test]
    fn levenshtein_bound_test() {
        assert_eq!(levenshtein_distance(&[], &["r", "u", "s", "t"]), 4);
        assert_eq!(
            levenshtein_distance(&['p', 'y', 't', 'h', 'o', 'n'], &[]),
            6
        );
        assert_eq!(levenshtein_distance(&vec![0; 0], &[0; 0]), 0);
    }
}
