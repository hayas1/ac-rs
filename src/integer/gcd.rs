#![allow(dead_code)]
#![allow(unused_imports)]

use num::integer::gcd;
use num::integer::lcm;
use num::Integer;

/// O(|v| log(min(v))) # calculate gcd recursively
fn gcd_recursive<T: Integer + Copy>(v: &Vec<T>) -> T {
    if v.len() == 0 {
        T::one()
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
        assert_eq!(gcd(128, 48), 16);
        assert_eq!(gcd(720, 600), 120);
        assert_eq!(gcd(38, 34), 2);
        assert_eq!(gcd(10, 15), 5);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(30, 45), 90);
        assert_eq!(lcm(128, 48), 384);
        assert_eq!(lcm(720, 600), 3600);
        assert_eq!(lcm(38, 34), 646);
        assert_eq!(lcm(10, 15), 30);
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
        assert_eq!(gcd_recursive(&Vec::<usize>::new()), 1);
        assert_eq!(gcd_recursive(&vec![12, 24]), 12);
        assert_eq!(gcd_recursive(&vec![12, 24, 32]), 4);
    }

    #[test]
    fn lcm_recursive_test() {
        assert_eq!(lcm_recursive(&vec![2, 3, 4]), 12);
        assert_eq!(lcm_recursive(&vec![12, 20, 32]), 480);
        assert_eq!(lcm_recursive(&vec![12, 20, 14, 32]), 3360);
    }
}
