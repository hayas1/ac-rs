use num::{Integer, Signed};

/// **O(log(b))** calculate a^b % modulo
pub fn mod_pow<T: Integer + Copy>(a: T, b: T, modulo: T) -> T {
    let two = T::one() + T::one();
    if b == T::zero() {
        T::one()
    } else if b % two == T::zero() {
        let half = mod_pow(a, b / two, modulo);
        half * half % modulo
    } else if b % two == T::one() {
        let half = mod_pow(a, b / two, modulo);
        half * half % modulo * a % modulo
    } else {
        unreachable!();
    }
}

/// **O(log(b))** calculate a^b % modulo
pub fn mod_pow_u64(a: u64, b: u64, modulo: u64) -> u64 {
    if b == 0 {
        1
    } else if b % 2 == 0 {
        let half = mod_pow_u64(a, b / 2, modulo);
        half * half % modulo
    } else if b % 2 == 1 {
        let half = mod_pow_u64(a, b / 2, modulo);
        half * half % modulo * a % modulo
    } else {
        unreachable!();
    }
}

/// **O(log(min(a, b)))** calculate pair (gcd(a,b), x, y) such that ax + by = gcd(a, b)
pub fn ex_euclid<T: Integer + Signed + Copy>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (gcd, xi, yi) = ex_euclid(b % a, a);
        (gcd, yi - b / a * xi, xi)
    }
}

/// **O(log(min(a, modulo)))** calculate inverse element of a in mod modulo multiplication
pub fn inverse_mod_mul<T: Integer + Signed + Copy>(a: T, modulo: T) -> Option<T> {
    if modulo == T::one() {
        None
    } else {
        let (gcd, inv, _y) = ex_euclid(a % modulo, modulo);
        match gcd == T::one() {
            true => Some((inv + modulo) % modulo),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        assert_eq!(mod_pow(10, 3, 1_000_000_007), 1000);
        assert_eq!(mod_pow(5, 3, 1_000_000_007), 125);
        assert_eq!(mod_pow(5, 4, 1_000_000_007), 625);
        assert_eq!(mod_pow(12, 2, 1_000_000_007), 144);
        assert_eq!(mod_pow(10, 3, 7), 6);
        assert_eq!(mod_pow(128, 3, 127), 1);
        assert_eq!(mod_pow(112134, 0, 132413247), 1);
    }

    #[test]
    fn bound_test() {
        assert_eq!(mod_pow(0, 10000000, 1_000_000_007), 0);
        assert_eq!(mod_pow(1, 1012351, 1_000_000_007), 1);
        assert_eq!(mod_pow(2, 10000000, 3), 1);
        assert_eq!(mod_pow(2, 9999999, 3), 2);
    }

    #[test]
    fn u64_basic_test() {
        assert_eq!(mod_pow_u64(10, 3, 1_000_000_007), 1000);
        assert_eq!(mod_pow_u64(5, 3, 1_000_000_007), 125);
        assert_eq!(mod_pow_u64(5, 4, 1_000_000_007), 625);
        assert_eq!(mod_pow_u64(12, 2, 1_000_000_007), 144);
        assert_eq!(mod_pow_u64(10, 3, 7), 6);
        assert_eq!(mod_pow_u64(128, 3, 127), 1);
        assert_eq!(mod_pow_u64(112134, 0, 132413247), 1);
    }

    #[test]
    fn u64_bound_test() {
        assert_eq!(mod_pow_u64(0, 10000000, 1_000_000_007), 0);
        assert_eq!(mod_pow_u64(1, 1012351, 1_000_000_007), 1);
        assert_eq!(mod_pow_u64(2, 10000000, 3), 1);
        assert_eq!(mod_pow_u64(2, 9999999, 3), 2);
    }

    #[test]
    fn ex_euclid_test() {
        assert_eq!(ex_euclid(3, 5), (1, 2, -1));
        assert_eq!(ex_euclid(6, 9), (3, -1, 1));
        assert_eq!(ex_euclid(32, 72), (8, -2, 1));
        assert_eq!(ex_euclid(10, 5), (5, 0, 1));
        for i in -100..100 {
            for j in -100..100 {
                let (gcd, x, y) = ex_euclid(i, j);
                assert_eq!(i * x + j * y, gcd);
            }
        }
    }

    #[test]
    fn inverse_mod_mul_test() {
        assert_eq!(inverse_mod_mul(3, 100), Some(67));
        assert_eq!(inverse_mod_mul(2, 4), None);
        assert_eq!(inverse_mod_mul(6, 9), None);
        assert_eq!(inverse_mod_mul(6, 13), Some(11));
        assert_eq!(inverse_mod_mul(19, 13), Some(11));
        assert_eq!(inverse_mod_mul(12, 7), Some(3));
        assert_eq!(inverse_mod_mul(1, 1), None);
        for i in 1..100 {
            for j in 1..100 {
                if let Some(inv) = inverse_mod_mul(i, j) {
                    assert_eq!(i * inv % j, 1);
                }
            }
        }
    }
}
