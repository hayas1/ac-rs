#![allow(dead_code)]

use num::Integer;

fn mod_pow<T: Integer + Copy>(a: T, b: T, modulo: T) -> T {
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
}
