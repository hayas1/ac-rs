// prime number is calculated as a index of Vec, so the type is usize

use std::collections::HashMap;

pub struct SieveOfEratosthenes {
    pub sieve: Vec<bool>,
}
impl SieveOfEratosthenes {
    /// **O(n log(log(n)))**, calculate n+1 size sieve, which vec[i] mean i is prime or not
    pub fn new(n: usize) -> Self {
        let mut sieve: Vec<_> = vec![true; n + 1];
        for i in (0..).take_while(|i| i * i <= n) {
            if i < 2 || !sieve[i] {
                sieve[i] = false;
                continue;
            }
            for j in 2..=(n / i) {
                sieve[i * j] = false;
            }
        }
        SieveOfEratosthenes { sieve }
    }

    /// **O(n)**, calculate vec of primes from 0 to max
    pub fn primes(&self) -> Vec<usize> {
        self.sieve.iter().enumerate().filter(|&(_i, &x)| x).map(|(i, _x)| i).collect()
    }

    /// **O(1)**, if num is prime then return true, else return false
    pub fn is_prime(&self, num: usize) -> bool {
        self.sieve[num]
    }
}

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
pub fn factorization(n: usize) -> HashMap<usize, usize> {
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

/// **O(n log(log(n)))**, calculate vec, which vec[i] mean min(factorization(n))
pub fn min_primes(size: usize) -> Vec<usize> {
    let mut sieve: Vec<_> = (0..=size).collect();
    for i in (2..).take_while(|i| i * i <= size) {
        for j in 2..=(size / i) {
            if sieve[i * j] == i * j {
                sieve[i * j] = i;
            } else {
                continue;
            }
        }
    }
    sieve
}

/// **O(log(n))**, calculate prime factorization of n, with min_primes
pub fn factorization_with_min_primes(n: usize, min_primes: &[usize]) -> HashMap<usize, usize> {
    if n <= 1 {
        return vec![(n, 1)].into_iter().collect();
    }
    let (mut divided, mut facts) = (n, HashMap::new());
    while divided > 1 {
        *facts.entry(min_primes[divided]).or_insert(0) += 1;
        divided /= min_primes[divided];
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_test0() {
        assert_eq!(SieveOfEratosthenes::new(0).sieve, vec![false]);
    }

    #[test]
    fn sieve_test1() {
        assert_eq!(SieveOfEratosthenes::new(1).sieve, vec![false, false]);
    }

    #[test]
    fn sieve_test2() {
        assert_eq!(SieveOfEratosthenes::new(2).sieve, vec![false, false, true]);
    }
    #[test]
    fn sieve_test30() {
        assert_eq!(
            SieveOfEratosthenes::new(30).sieve,
            vec![
                false, false, true, true, false, true, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, false, false, true,
                false, false, false, false, false, true, false
            ]
        );
    }
    #[test]
    fn sieve_test100() {
        assert_eq!(
            SieveOfEratosthenes::new(100).sieve,
            vec![
                false, false, true, true, false, true, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, false, false, true,
                false, false, false, false, false, true, false, true, false, false, false, false,
                false, true, false, false, false, true, false, true, false, false, false, true,
                false, false, false, false, false, true, false, false, false, false, false, true,
                false, true, false, false, false, false, false, true, false, false, false, true,
                false, true, false, false, false, false, false, true, false, false, false, true,
                false, false, false, false, false, true, false, false, false, false, false, false,
                false, true, false, false, false
            ]
        );
    }

    #[test]
    fn primes_test30() {
        assert_eq!(SieveOfEratosthenes::new(30).primes(), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn primes_test100() {
        assert_eq!(
            SieveOfEratosthenes::new(100).primes(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn is_prime_test() {
        let sieve = SieveOfEratosthenes::new(100);
        assert_eq!(sieve.is_prime(2), true);
        assert_eq!(sieve.is_prime(3), true);
        assert_eq!(sieve.is_prime(4), false);
        assert_eq!(sieve.is_prime(5), true);
        assert_eq!(sieve.is_prime(13), true);
        assert_eq!(sieve.is_prime(15), false);
        assert_eq!(sieve.is_prime(17), true);
        assert_eq!(sieve.is_prime(57), false);
        assert_eq!(sieve.is_prime(83), true);
        assert_eq!(sieve.is_prime(91), false);
    }

    #[test]
    fn fast_sieve_test0() {
        assert_eq!(fast_primes(0), vec![]);
    }

    #[test]
    fn fast_sieve_test1() {
        assert_eq!(fast_primes(1), vec![]);
    }

    #[test]
    fn fast_sieve_test2() {
        assert_eq!(fast_primes(2), vec![2]);
    }

    #[test]
    fn fast_sieve_test30() {
        assert_eq!(fast_primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn fast_sieve_test100() {
        assert_eq!(
            fast_primes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn min_primes_test30() {
        assert_eq!(
            min_primes(30),
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2
            ]
        );
    }
    #[test]
    fn min_primes_test100() {
        assert_eq!(
            min_primes(100),
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2, 31, 2, 3, 2, 5, 2, 37, 2, 3, 2, 41, 2, 43, 2, 3, 2, 47, 2, 7, 2, 3,
                2, 53, 2, 5, 2, 3, 2, 59, 2, 61, 2, 3, 2, 5, 2, 67, 2, 3, 2, 71, 2, 73, 2, 3, 2, 7,
                2, 79, 2, 3, 2, 83, 2, 5, 2, 3, 2, 89, 2, 7, 2, 3, 2, 5, 2, 97, 2, 3, 2
            ]
        );
    }
    #[test]
    fn factorization_test01() {
        assert_eq!(factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn factorization_test() {
        assert_eq!(factorization(2), vec![(2, 1)].into_iter().collect());
        assert_eq!(factorization(4), vec![(2, 2)].into_iter().collect());
        assert_eq!(factorization(8), vec![(2, 3)].into_iter().collect());
        assert_eq!(factorization(16), vec![(2, 4)].into_iter().collect());
        assert_eq!(factorization(15), vec![(3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(60), vec![(2, 2), (3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(300), vec![(2, 2), (3, 1), (5, 2)].into_iter().collect());
    }

    #[test]
    fn factorization_with_min_primes_test01() {
        let p = min_primes(100);
        assert_eq!(factorization_with_min_primes(0, &p), vec![(0, 1)].into_iter().collect());
        assert_eq!(factorization_with_min_primes(1, &p), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn factorization_with_min_primes_test() {
        let p = min_primes(300);
        assert_eq!(factorization_with_min_primes(2, &p), vec![(2, 1)].into_iter().collect());
        assert_eq!(factorization_with_min_primes(4, &p), vec![(2, 2)].into_iter().collect());
        assert_eq!(factorization_with_min_primes(8, &p), vec![(2, 3)].into_iter().collect());
        assert_eq!(factorization_with_min_primes(16, &p), vec![(2, 4)].into_iter().collect());
        assert_eq!(
            factorization_with_min_primes(15, &p),
            vec![(3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(60, &p),
            vec![(2, 2), (3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(300, &p),
            vec![(2, 2), (3, 1), (5, 2)].into_iter().collect()
        );
    }
}
