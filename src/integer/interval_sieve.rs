/// prime number is calculated as a index of Vec, so their type is usize
pub struct IntervalSieve {
    pub l: usize,
    pub r: usize,
    pub base: Vec<usize>,
    pub interval: Vec<bool>,
}
impl IntervalSieve {
    /// **O((r - l)log(log(r)) + sqrt(r))*, Sieve primes in the interval `[l, r)`.
    pub fn new(l: usize, r: usize) -> Self {
        let cap = (r as f64).sqrt() as usize + 1;
        let mut base: Vec<_> = (0..=cap).collect();
        let mut interval = vec![true; r - l + 1];

        for i in 2..=cap {
            if base[i] != i {
                continue;
            }
            for j in (2 * i..=cap).step_by(i) {
                base[j] = base[j].min(i);
            }

            let mut p = (l + i - 1) / i * i;
            if p == i {
                p = i * 2;
            }
            for q in (p..=r).step_by(i) {
                interval[q - l] = false;
            }
        }
        IntervalSieve { l, r, base, interval }
    }

    /// **O(1)**, if num is prime then return true, else return false
    pub fn is_prime(&self, n: usize) -> bool {
        if self.l <= n && n <= self.r {
            self.interval[n - self.l]
        } else {
            if n < self.base.len() {
                return n > 1 && self.base[n] == n;
            } else {
                todo!()
            }
        }
    }

    /// **O(n)**, calculate vec of primes from `[l, r]`
    pub fn primes(&self) -> Vec<usize> {
        (self.l..=self.r).filter(|&n| self.is_prime(n)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_50_60() {
        let sieve = IntervalSieve::new(50, 60);

        assert_eq!(sieve.base, vec![0, 1, 2, 3, 2, 5, 2, 7, 2]);
        assert_eq!(
            sieve.interval,
            vec![false, false, false, true, false, false, false, false, false, true, false]
        );
    }

    #[test]
    fn test_is_prime_16_25() {
        let sieve = IntervalSieve::new(16, 25);

        assert!(!sieve.is_prime(16));
        assert!(sieve.is_prime(17));
        assert!(!sieve.is_prime(18));
        assert!(sieve.is_prime(19));
        assert!(!sieve.is_prime(20));
        assert!(sieve.is_prime(23));
        assert!(!sieve.is_prime(24));
        assert!(!sieve.is_prime(25));
    }

    #[test]
    fn test_primes_10_20() {
        let sieve = IntervalSieve::new(10, 20);
        let primes = sieve.primes();

        assert_eq!(primes, vec![11, 13, 17, 19]);
    }
}
