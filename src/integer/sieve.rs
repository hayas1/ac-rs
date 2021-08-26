use std::collections::HashMap;

/// prime number is calculated as a index of Vec, so their type is usize
pub struct SieveOfEratosthenes {
    pub min_primes: Vec<usize>,
}
impl SieveOfEratosthenes {
    /// **O(n log(log(n)))**, calculate size n+1 sieve, which vec[i] mean minimum prime of i's devisor
    pub fn new(n: usize) -> Self {
        let mut min_primes: Vec<_> = (0..=n).collect();
        for i in (2..).take_while(|i| i * i <= n) {
            // only if minimum prime of i's devisor is i, the sieve is updated
            for p in 2..=(n / i * (min_primes[i] == i) as usize) {
                min_primes[i * p] = min_primes[i * p].min(i);
            }
        }
        SieveOfEratosthenes { min_primes }
    }

    /// **O(1)**, if num is prime then return true, else return false
    pub fn is_prime(&self, num: usize) -> bool {
        num > 1 && num == self.min_primes[num]
    }

    /// **O(n)**, returned vec[i] mean i is prime or not
    pub fn sieve(&self) -> Vec<bool> {
        (0..self.min_primes.len()).map(|x| self.is_prime(x)).collect()
    }

    /// **O(n)**, calculate vec of primes from 0 to max
    pub fn primes(&self) -> Vec<usize> {
        (0..self.min_primes.len()).filter(|&x| self.is_prime(x)).map(|x| x).collect()
    }

    /// **O(log(n))**, calculate prime factorization of n, with min_primes
    pub fn factorization(&self, n: usize) -> HashMap<usize, usize> {
        let (mut divided, mut facts) = (n, HashMap::new());
        while facts.is_empty() || divided > 1 {
            *facts.entry(self.min_primes[divided]).or_insert(0) += 1;
            divided /= self.min_primes[divided].max(1);
        }
        facts
    }

    /// **O(log(n))**, the number of integers that are prime to n each other less than n
    pub fn euler_phi(&self, n: usize) -> usize {
        if n < 2 {
            return n;
        }
        let (numerator, denominator) =
            self.factorization(n).keys().fold((1, 1), |(pn, pd), p| (pn * (p - 1), pd * p));
        n * numerator / denominator
    }
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

    #[test]
    fn euler_phi_bound_test() {
        let eratosthenes = SieveOfEratosthenes::new(0);
        assert_eq!(eratosthenes.euler_phi(0), 0);
        let eratosthenes = SieveOfEratosthenes::new(1);
        assert_eq!(eratosthenes.euler_phi(0), 0);
        assert_eq!(eratosthenes.euler_phi(1), 1);
        let eratosthenes = SieveOfEratosthenes::new(2);
        assert_eq!(eratosthenes.euler_phi(0), 0);
        assert_eq!(eratosthenes.euler_phi(1), 1);
        assert_eq!(eratosthenes.euler_phi(2), 1);
    }

    #[test]
    fn euler_phi_test() {
        let eratosthenes = SieveOfEratosthenes::new(300);
        assert_eq!(eratosthenes.euler_phi(3), 2);
        assert_eq!(eratosthenes.euler_phi(6), 2);
        assert_eq!(eratosthenes.euler_phi(5), 4);
        assert_eq!(eratosthenes.euler_phi(12), 4);
        assert_eq!(eratosthenes.euler_phi(300), 80);
    }
}
