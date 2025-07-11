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
        (0..self.min_primes.len()).filter(|&x| self.is_prime(x)).collect()
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

    /// **O(n log(log(n)))**, calculate all prime factorization of numbers from 0 to n
    pub fn all_factorization(&self) -> Vec<HashMap<usize, usize>> {
        let n = self.min_primes.len();
        let mut divided: Vec<_> = (0..n).collect();
        let mut facts = vec![HashMap::new(); n];
        for p in self.primes() {
            for i in (p..n).step_by(p) {
                let mut count = 0;
                while divided[i] % p == 0 {
                    divided[i] /= p;
                    count += 1;
                }
                if count > 0 {
                    *facts[i].entry(p).or_insert(0) += count;
                }
            }
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
    fn test_min_primes30() {
        assert_eq!(
            SieveOfEratosthenes::new(30).min_primes,
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2
            ]
        );
    }
    #[test]
    fn test_min_primes100() {
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
    fn test_sieve_bound0() {
        assert_eq!(SieveOfEratosthenes::new(0).sieve(), vec![false]);
        assert_eq!(SieveOfEratosthenes::new(1).sieve(), vec![false, false]);
        assert_eq!(SieveOfEratosthenes::new(2).sieve(), vec![false, false, true]);
    }

    #[test]
    fn test_sieve30() {
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
    fn test_sieve100() {
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
    fn test_primes30() {
        assert_eq!(SieveOfEratosthenes::new(30).primes(), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_primes100() {
        assert_eq!(
            SieveOfEratosthenes::new(100).primes(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn test_is_prime() {
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
    fn test_factorization1() {
        let eratosthenes = SieveOfEratosthenes::new(100);
        assert_eq!(eratosthenes.factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(eratosthenes.factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn test_factorization2() {
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
    fn test_all_factorization() {
        let eratosthenes = SieveOfEratosthenes::new(30);
        let facts = eratosthenes.all_factorization();
        assert_eq!(facts[0], HashMap::new());
        assert_eq!(facts[1], HashMap::new());
        assert_eq!(facts[2], vec![(2, 1)].into_iter().collect());
        assert_eq!(facts[3], vec![(3, 1)].into_iter().collect());
        assert_eq!(facts[4], vec![(2, 2)].into_iter().collect());
        assert_eq!(facts[5], vec![(5, 1)].into_iter().collect());
        assert_eq!(facts[6], vec![(2, 1), (3, 1)].into_iter().collect());
        assert_eq!(facts[7], vec![(7, 1)].into_iter().collect());
        assert_eq!(facts[8], vec![(2, 3)].into_iter().collect());
        assert_eq!(facts[9], vec![(3, 2)].into_iter().collect());
        assert_eq!(facts[10], vec![(2, 1), (5, 1)].into_iter().collect());
        assert_eq!(facts[11], vec![(11, 1)].into_iter().collect());
        assert_eq!(facts[12], vec![(2, 2), (3, 1)].into_iter().collect());
        assert_eq!(facts[13], vec![(13, 1)].into_iter().collect());
        assert_eq!(facts[14], vec![(2, 1), (7, 1)].into_iter().collect());
        assert_eq!(facts[15], vec![(3, 1), (5, 1)].into_iter().collect());
        assert_eq!(facts[16], vec![(2, 4)].into_iter().collect());
        assert_eq!(facts[17], vec![(17, 1)].into_iter().collect());
        assert_eq!(facts[18], vec![(2, 1), (3, 2)].into_iter().collect());
        assert_eq!(facts[19], vec![(19, 1)].into_iter().collect());
        assert_eq!(facts[20], vec![(2, 2), (5, 1)].into_iter().collect());
        assert_eq!(facts[21], vec![(3, 1), (7, 1)].into_iter().collect());
        assert_eq!(facts[22], vec![(2, 1), (11, 1)].into_iter().collect());
        assert_eq!(facts[23], vec![(23, 1)].into_iter().collect());
        assert_eq!(facts[24], vec![(2, 3), (3, 1)].into_iter().collect());
        assert_eq!(facts[25], vec![(5, 2)].into_iter().collect());
        assert_eq!(facts[26], vec![(2, 1), (13, 1)].into_iter().collect());
        assert_eq!(facts[27], vec![(3, 3)].into_iter().collect());
        assert_eq!(facts[28], vec![(2, 2), (7, 1)].into_iter().collect());
        assert_eq!(facts[29], vec![(29, 1)].into_iter().collect());
        assert_eq!(facts[30], vec![(2, 1), (3, 1), (5, 1)].into_iter().collect());
    }

    #[test]
    fn test_euler_phi_bound() {
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
    fn test_euler_phi() {
        let eratosthenes = SieveOfEratosthenes::new(300);
        assert_eq!(eratosthenes.euler_phi(3), 2);
        assert_eq!(eratosthenes.euler_phi(6), 2);
        assert_eq!(eratosthenes.euler_phi(5), 4);
        assert_eq!(eratosthenes.euler_phi(12), 4);
        assert_eq!(eratosthenes.euler_phi(300), 80);
    }
}
