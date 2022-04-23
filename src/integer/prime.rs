use std::collections::HashMap;

/// **O(n)...?**, calculate vec of primes from 0 to max
pub fn fast_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let (mut is_prime, mut min_primes) = (vec![true; n + 1], vec![0; n + 1]);
    for i in 0..=n {
        if i < 2 {
            is_prime[i] = false;
        }
        if is_prime[i] {
            primes.push(i);
            min_primes[i] = i;
        }
        for &p in primes.iter() {
            if i * p > n || p > min_primes[i] {
                break;
            }
            is_prime[i * p] = false;
            min_primes[i * p] = p;
        }
    }
    primes
}

/// **O(sqrt(n))**, calculate prime factorization of n
pub fn factorization(n: u64) -> HashMap<u64, usize> {
    if n < 2 {
        return vec![(n, 1)].into_iter().collect();
    }
    let (mut divided, mut facts) = (n, HashMap::new());
    for i in (2..).take_while(|i| i * i <= n) {
        while divided % i == 0 {
            divided /= i;
            *facts.entry(i).or_insert(0) += 1;
        }
    }
    if divided > 1 {
        *facts.entry(divided).or_insert(0) += 1;
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_sieve0() {
        assert_eq!(fast_primes(0), vec![]);
    }

    #[test]
    fn test_fast_sieve1() {
        assert_eq!(fast_primes(1), vec![]);
    }

    #[test]
    fn test_fast_sieve2() {
        assert_eq!(fast_primes(2), vec![2]);
    }

    #[test]
    fn test_fast_sieve30() {
        assert_eq!(fast_primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_fast_sieve100() {
        assert_eq!(
            fast_primes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn test_factorization_bound1() {
        assert_eq!(factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn test_factorization_bound2() {
        assert_eq!(factorization(2), vec![(2, 1)].into_iter().collect());
        assert_eq!(factorization(4), vec![(2, 2)].into_iter().collect());
        assert_eq!(factorization(8), vec![(2, 3)].into_iter().collect());
        assert_eq!(factorization(16), vec![(2, 4)].into_iter().collect());
        assert_eq!(factorization(15), vec![(3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(60), vec![(2, 2), (3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(300), vec![(2, 2), (3, 1), (5, 2)].into_iter().collect());
    }
}
