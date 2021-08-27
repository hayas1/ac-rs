use num::{Integer, NumCast};

pub struct Counting<T: Integer> {
    p: T,
    fac: Vec<T>,
    fac_inv: Vec<T>,
}
impl<T: Integer + Copy + NumCast> Counting<T> {
    /// **O(n)**, ready to compute combination(n,k) mod p, where p is prime and larger than n
    pub fn new(max_n: usize, p: T) -> Self {
        let (mut fac, mut fac_inv, mut inv) =
            (vec![T::one(); max_n + 1], vec![T::one(); max_n + 1], vec![T::one(); max_n + 1]);
        for i in 2..=max_n {
            let i_as_t = T::from(i).expect("i is smaller than max_n");
            let p_mod_i = (p % i_as_t).to_usize().expect("i is smaller than max_n");
            inv[i] = p - inv[p_mod_i] * (p / i_as_t) % p;
            fac[i] = fac[i - 1] % p * i_as_t % p;
            fac_inv[i] = fac_inv[i - 1] * inv[i] % p;
        }
        Counting { p, fac, fac_inv }
    }

    /// **O(1)**, compute n! mod p
    pub fn factorial(&self, n: usize) -> T {
        self.fac[n]
    }

    /// **O(1)**, compute nPk mod p
    pub fn permutation(&self, n: usize, k: usize) -> T {
        if n < k {
            T::zero()
        } else {
            self.fac[n] * self.fac_inv[n - k] % self.p
        }
    }

    /// **O(1)**, compute nCk mod p
    pub fn combination(&self, n: usize, k: usize) -> T {
        if n < k {
            T::zero()
        } else {
            self.fac[n] * (self.fac_inv[k] * self.fac_inv[n - k] % self.p) % self.p
        }
    }

    /// **O(1)**, compute nHk mod p
    pub fn combination_with_repetition(&self, n: usize, k: usize) -> T {
        self.combination(n + k - 1, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        let c = Counting::new(100, 1_000_000_007u64);
        assert_eq!(c.factorial(0), 1);
        assert_eq!(c.factorial(1), 1);
        assert_eq!(c.factorial(2), 2);
        assert_eq!(c.factorial(3), 6);
        assert_eq!(c.factorial(4), 24);
        assert_eq!(c.factorial(5), 120);
        assert_eq!(c.factorial(10), 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);
        assert_eq!(c.factorial(100), 437918130);
    }

    #[test]
    fn permutation_basic_test() {
        let c = Counting::new(100, 1_000_000_007u64);
        assert_eq!(c.permutation(10, 3), 720);
        assert_eq!(c.permutation(12, 4), 11880);
        assert_eq!(c.permutation(18, 6), 13366080);
        assert_eq!(c.permutation(100, 3), 970200);
    }

    #[test]
    fn permutation_bound_test() {
        let c0 = Counting::new(0, 1_000_000_007);
        assert_eq!(c0.permutation(0, 0), 1);
        let c1 = Counting::new(1, 1_000_000_007);
        assert_eq!(c1.permutation(0, 0), 1);
        assert_eq!(c1.permutation(1, 0), 1);
        assert_eq!(c1.permutation(1, 1), 1);
        assert_eq!(c1.permutation(0, 1), 0);
    }

    #[test]
    fn combination_basic_test() {
        let c = Counting::new(100, 1_000_000_007usize);
        assert_eq!(c.combination(10, 3), 120);
        assert_eq!(c.combination(12, 4), 12 * 11 * 10 * 9 / 4 / 3 / 2 / 1);
        assert_eq!(c.combination(18, 5), 8568);
        assert_eq!(c.combination(100, 3), 100 * 33 * 49);
    }

    #[test]
    fn combination_large_test() {
        let c = Counting::new(100, 9007199254740997u128);
        assert_eq!(c.combination(10, 3), 120);
        assert_eq!(c.combination(12, 4), 12 * 11 * 10 * 9 / 4 / 3 / 2 / 1);
        assert_eq!(c.combination(18, 5), 8568);
        assert_eq!(c.combination(100, 3), 100 * 33 * 49);
    }

    #[test]
    fn combination_bound_test() {
        let c0 = Counting::new(0, 1_000_000_007);
        assert_eq!(c0.combination(0, 0), 1);
        let c1 = Counting::new(1, 1_000_000_007);
        assert_eq!(c1.combination(0, 0), 1);
        assert_eq!(c1.combination(1, 0), 1);
        assert_eq!(c1.combination(1, 1), 1);
        assert_eq!(c1.combination(0, 1), 0);
    }

    #[test]
    fn combination_with_repetition_test() {
        let c = Counting::new(100, 1_000_000_007u128);
        assert_eq!(c.combination_with_repetition(3, 2), 6);
        assert_eq!(c.combination_with_repetition(3, 5), 21);
        assert_eq!(c.combination_with_repetition(7, 3), 84);
    }

    #[test]
    fn property_test() {
        let c = Counting::new(1000, 1_000_000_007i64);
        for i in 0..=1000 {
            for j in 0..=i {
                assert_eq!(c.combination(i, j), c.combination(i, i - j));
            }
        }
    }
}
