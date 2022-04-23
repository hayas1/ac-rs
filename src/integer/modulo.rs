use num::{Integer, Signed};

/// **O(log(b))**, calculate a^b % modulo
pub fn mod_pow<T: Integer + Copy>(a: T, b: T, modulo: T) -> T {
    let two = T::one() + T::one();
    if b == T::zero() {
        T::one()
    } else if b % two == T::zero() {
        let half_pow = mod_pow(a, b / two, modulo);
        half_pow * half_pow % modulo
    } else if b % two == T::one() {
        let half_pow = mod_pow(a, b / two, modulo);
        half_pow * half_pow % modulo * a % modulo
    } else {
        unreachable!();
    }
}

/// **O(log(b))**, calculate a^b % modulo
pub fn mod_pow_u64(a: u64, b: u64, modulo: u64) -> u64 {
    if b == 0 {
        1
    } else {
        let half_pow = mod_pow_u64(a, b / 2, modulo);
        match b % 2 {
            0 => half_pow * half_pow % modulo,
            1 => half_pow * half_pow % modulo * a % modulo,
            _ => unreachable!(),
        }
    }
}

/// **O(log(min(a, b)))**, calculate pair (gcd(a,b), x, y) such that ax + by = gcd(a, b)
pub fn ex_euclid<T: Integer + Signed + Copy>(a: T, b: T) -> ((T, T), T) {
    if a == T::zero() {
        ((T::zero(), T::one()), b)
    } else {
        let ((xi, yi), gcd) = ex_euclid(b % a, a);
        ((yi - b / a * xi, xi), gcd)
    }
}

/// **O(log(min(a, modulo)))**, calculate inverse element of a in mod modulo multiplication
pub fn inverse_mod_mul<T: Integer + Signed + Copy>(a: T, modulo: T) -> Option<T> {
    if modulo == T::one() {
        None
    } else {
        let ((inv, _), gcd) = ex_euclid(a % modulo, modulo);
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
    fn test_basic() {
        assert_eq!(mod_pow(10, 3, 1_000_000_007), 1000);
        assert_eq!(mod_pow(5, 3, 1_000_000_007), 125);
        assert_eq!(mod_pow(5, 4, 1_000_000_007), 625);
        assert_eq!(mod_pow(12, 2, 1_000_000_007), 144);
        assert_eq!(mod_pow(10, 3, 7), 6);
        assert_eq!(mod_pow(128, 3, 127), 1);
        assert_eq!(mod_pow(112134, 0, 132413247), 1);
    }

    #[test]
    fn test_bound() {
        assert_eq!(mod_pow(0, 10000000, 1_000_000_007), 0);
        assert_eq!(mod_pow(1, 1012351, 1_000_000_007), 1);
        assert_eq!(mod_pow(2, 10000000, 3), 1);
        assert_eq!(mod_pow(2, 9999999, 3), 2);
    }

    #[test]
    fn test_u64_basic() {
        assert_eq!(mod_pow_u64(10, 3, 1_000_000_007), 1000);
        assert_eq!(mod_pow_u64(5, 3, 1_000_000_007), 125);
        assert_eq!(mod_pow_u64(5, 4, 1_000_000_007), 625);
        assert_eq!(mod_pow_u64(12, 2, 1_000_000_007), 144);
        assert_eq!(mod_pow_u64(10, 3, 7), 6);
        assert_eq!(mod_pow_u64(128, 3, 127), 1);
        assert_eq!(mod_pow_u64(112134, 0, 132413247), 1);
    }

    #[test]
    fn test_u64_bound() {
        assert_eq!(mod_pow_u64(0, 10000000, 1_000_000_007), 0);
        assert_eq!(mod_pow_u64(1, 1012351, 1_000_000_007), 1);
        assert_eq!(mod_pow_u64(2, 10000000, 3), 1);
        assert_eq!(mod_pow_u64(2, 9999999, 3), 2);
    }

    #[test]
    fn test_ex_euclid() {
        assert_eq!(ex_euclid(3, 5), ((2, -1), 1));
        assert_eq!(ex_euclid(6, 9), ((-1, 1), 3));
        assert_eq!(ex_euclid(32, 72), ((-2, 1), 8));
        assert_eq!(ex_euclid(10, 5), ((0, 1), 5));
        for i in -100..100 {
            for j in -100..100 {
                let ((x, y), gcd) = ex_euclid(i, j);
                assert_eq!(i * x + j * y, gcd);
            }
        }
    }

    #[test]
    fn test_inverse_mod_mul() {
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
