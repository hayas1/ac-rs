// prime number is calculated as a index of Vec, so the type is usize

use std::collections::HashMap;

pub struct SieveOfEratosthenes {
    pub min_primes: Vec<usize>,
}
impl SieveOfEratosthenes {
    /// **O(n log(log(n)))**, calculate n+1 size sieve, which vec[i] mean minimum prime of i's devisor
    pub fn new(n: usize) -> Self {
        let mut min_primes: Vec<_> = (0..=n).collect();
        for i in (0..).take_while(|i| i * i <= n) {
            if i < 2 || min_primes[i] != i {
                continue;
            }
            for j in 2..=(n / i) {
                if min_primes[i * j] == i * j {
                    min_primes[i * j] = i;
                }
            }
        }
        SieveOfEratosthenes { min_primes }
    }

    /// **O(1)**, if num is prime then return true, else return false
    pub fn is_prime(&self, num: usize) -> bool {
        num > 1 && num == self.min_primes[num]
    }

    /// **O(n)**, calculate n+1 size vec, which vec[i] mean i is prime or not
    pub fn sieve(&self) -> Vec<bool> {
        (0..self.min_primes.len()).map(|x| self.is_prime(x)).collect()
    }

    /// **O(n)**, calculate vec of primes from 0 to max
    pub fn primes(&self) -> Vec<usize> {
        (0..self.min_primes.len()).filter(|&x| self.is_prime(x)).map(|x| x).collect()
    }

    /// **O(log(n))**, calculate prime factorization of n, with min_primes
    pub fn factorization(&self, n: usize) -> HashMap<usize, usize> {
        if n < 2 {
            return vec![(n, 1)].into_iter().collect();
        }
        let (mut divided, mut facts) = (n, HashMap::new());
        while divided > 1 {
            *facts.entry(self.min_primes[divided]).or_insert(0) += 1;
            divided /= self.min_primes[divided];
        }
        facts
    }

    /// **O(log(n))**, the number of integers that are prime to n each other less than n
    pub fn euler_phi(&self, n: usize) -> usize {
        if n < 2 {
            return 0;
        }
        let (numerator, denominator) =
            self.factorization(n).iter().fold((1, 1), |(n, d), (p, _c)| (n * (p - 1), d * (p)));
        n * numerator / denominator
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_primes_test30() {
        assert_eq!(
            SieveOfEratosthenes::new(30).min_primes,
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2
            ]
        );
    }
    #[test]
    fn min_primes_test100() {
        assert_eq!(
            SieveOfEratosthenes::new(100).min_primes,
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2, 31, 2, 3, 2, 5, 2, 37, 2, 3, 2, 41, 2, 43, 2, 3, 2, 47, 2, 7, 2, 3,
                2, 53, 2, 5, 2, 3, 2, 59, 2, 61, 2, 3, 2, 5, 2, 67, 2, 3, 2, 71, 2, 73, 2, 3, 2, 7,
                2, 79, 2, 3, 2, 83, 2, 5, 2, 3, 2, 89, 2, 7, 2, 3, 2, 5, 2, 97, 2, 3, 2
            ]
        );
    }

    #[test]
    fn sieve_bound_test0() {
        assert_eq!(SieveOfEratosthenes::new(0).sieve(), vec![false]);
        assert_eq!(SieveOfEratosthenes::new(1).sieve(), vec![false, false]);
        assert_eq!(SieveOfEratosthenes::new(2).sieve(), vec![false, false, true]);
    }

    #[test]
    fn sieve_test30() {
        assert_eq!(
            SieveOfEratosthenes::new(30).sieve(),
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
            SieveOfEratosthenes::new(100).sieve(),
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
    fn factorization_bound_test1() {
        assert_eq!(factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn factorization_bound_test2() {
        assert_eq!(factorization(2), vec![(2, 1)].into_iter().collect());
        assert_eq!(factorization(4), vec![(2, 2)].into_iter().collect());
        assert_eq!(factorization(8), vec![(2, 3)].into_iter().collect());
        assert_eq!(factorization(16), vec![(2, 4)].into_iter().collect());
        assert_eq!(factorization(15), vec![(3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(60), vec![(2, 2), (3, 1), (5, 1)].into_iter().collect());
        assert_eq!(factorization(300), vec![(2, 2), (3, 1), (5, 2)].into_iter().collect());
    }

    #[test]
    fn factorization_test1() {
        let eratosthenes = SieveOfEratosthenes::new(100);
        assert_eq!(eratosthenes.factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn factorization_test2() {
        let eratosthenes = SieveOfEratosthenes::new(300);
        assert_eq!(eratosthenes.factorization(2), vec![(2, 1)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(4), vec![(2, 2)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(8), vec![(2, 3)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(16), vec![(2, 4)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(15), vec![(3, 1), (5, 1)].into_iter().collect());
        assert_eq!(
            eratosthenes.factorization(60),
            vec![(2, 2), (3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            eratosthenes.factorization(300),
            vec![(2, 2), (3, 1), (5, 2)].into_iter().collect()
        );
    }
}
