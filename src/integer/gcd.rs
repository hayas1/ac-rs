#![allow(dead_code)]
#![allow(unused_imports)]

use num::integer::gcd;
use num::integer::lcm;
use num::Integer;

/// O(|v| log(min(v))) # calculate gcd recursively
fn gcd_recursive<T: Integer + Copy>(v: &Vec<T>) -> T {
    if v.len() == 0 {
        T::zero()
    } else if v.len() == 1 {
        v[0]
    } else {
        let mid = (v.len() + 1) / 2;
        gcd_recursive(
            &(0..mid)
                .map(|i| {
                    if 2 * i < v.len() - 1 {
                        gcd(v[2 * i], v[2 * i + 1])
                    } else {
                        v[2 * i]
                    }
                })
                .collect(),
        )
    }
}

/// O(|v| log(min(v))) # calculate lcm recursively
fn lcm_recursive<T: Integer + Copy>(v: &Vec<T>) -> T {
    if v.len() == 0 {
        T::one()
    } else if v.len() == 1 {
        v[0]
    } else {
        let mid = (v.len() + 1) / 2;
        lcm_recursive(
            &(0..mid)
                .map(|i| {
                    if 2 * i < v.len() - 1 {
                        lcm(v[2 * i], v[2 * i + 1])
                    } else {
                        v[2 * i]
                    }
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(30, 45), 15);
        assert_eq!(gcd(16, 0), 16);
        assert_eq!(gcd(0, 16), 16);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(16, 1), 1);
        assert_eq!(gcd(1, 16), 1);
        assert_eq!(gcd(1, 1), 1);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(30, 45), 90);
        assert_eq!(lcm(32, 1), 32);
        assert_eq!(lcm(1, 32), 32);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(0, 2), 0);
        assert_eq!(lcm(2, 0), 0);
        assert_eq!(lcm(0, 0), 0)
    }

    #[test]
    fn gcd_recursive_test() {
        assert_eq!(gcd_recursive(&vec![12, 20, 32]), 4);
        assert_eq!(gcd_recursive(&vec![12, 20, 14, 32]), 2);
    }

    #[test]
    fn gcd_recursive_test_b() {
        assert_eq!(gcd_recursive(&vec![12, 20, 32, 91]), 1);
        assert_eq!(gcd_recursive(&vec![12]), 12);
        assert_eq!(gcd_recursive(&Vec::<usize>::new()), 0);
        assert_eq!(gcd_recursive(&vec![12, 24]), 12);
        assert_eq!(gcd_recursive(&vec![12, 24, 32]), 4);
    }

    #[test]
    fn lcm_recursive_test() {
        assert_eq!(lcm_recursive(&vec![2, 3, 4]), 12);
        assert_eq!(lcm_recursive(&vec![12, 20, 32]), 480);
        assert_eq!(lcm_recursive(&vec![12, 20, 14, 32]), 3360);
        assert_eq!(lcm_recursive(&vec![12]), 12);
        assert_eq!(lcm_recursive(&Vec::<usize>::new()), 1);
    }
}
