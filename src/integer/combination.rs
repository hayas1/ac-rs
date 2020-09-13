#![allow(dead_code)]

struct Combination {
    modulo: usize,
    fac: Vec<usize>,
    fac_inv: Vec<usize>,
    inv: Vec<usize>,
}
impl Combination {
    /// O(n) # ready to compute combination(n,k) mod p, where p is prime and larger than n
    fn new(max: usize, modulo: usize) -> Self {
        let (mut fac, mut fac_inv, mut inv) =
            (vec![1; max + 1], vec![1; max + 1], vec![1; max + 1]);
        for i in 2..=max {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fac_inv[i] = fac_inv[i - 1] * inv[i] % modulo;
        }
        Combination {
            modulo,
            fac,
            fac_inv,
            inv,
        }
    }

    /// O(1) # compute combination(n,k) mod p, where p is prime
    fn combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.fac_inv[k] * self.fac_inv[n - k] % self.modulo) % self.modulo
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        let c = Combination::new(100, 1_000_000_007);
        assert_eq!(c.combination(10, 3), 120);
        assert_eq!(c.combination(12, 4), 12 * 11 * 10 * 9 / 4 / 3 / 2 / 1);
        assert_eq!(c.combination(18, 5), 8568);
        assert_eq!(c.combination(100, 3), 100 * 33 * 49);
    }

    #[test]
    fn bound_test() {
        let c0 = Combination::new(0, 1_000_000_007);
        assert_eq!(c0.combination(0, 0), 1);
        let c1 = Combination::new(1, 1_000_000_007);
        assert_eq!(c1.combination(0, 0), 1);
        assert_eq!(c1.combination(1, 0), 1);
        assert_eq!(c1.combination(1, 1), 1);
    }

    #[test]
    fn property_test() {
        let c = Combination::new(1000, 1_000_000_007);
        for i in 0..=1000 {
            for j in 0..=i {
                assert_eq!(c.combination(i, j), c.combination(i, i - j))
            }
        }
    }
}
