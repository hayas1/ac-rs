#![allow(dead_code)]
// prime number is calculated as a index of Vec, so the type is usize

use std::collections::HashMap;

/// **O(n log(log(n)))** calculate n size vec, which vec[i] mean i is prime or not, with sieve of Eratosthenes
fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut sieve: Vec<_> = (0..=n).collect();
    for i in (1..).take_while(|i| i * i <= n) {
        for j in 2..=(n / i) {
            if sieve[i * j] % i == 0 {
                sieve[i * j] /= i;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .map(|(i, &x)| i > 1 && i == x)
        .collect()
}

/// **O(n log(log(n)))** calculate vec of primes from 0 to max
fn primes(max: usize) -> Vec<usize> {
    sieve_of_eratosthenes(max)
        .iter()
        .enumerate()
        .filter(|&(_i, &x)| x)
        .map(|(i, _x)| i)
        .collect()
}

/// **O(n)...?** calculate vec of primes from 0 to max
fn fast_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let (mut is_prime, mut min_primes) = (vec![true; n + 1], vec![0; n + 1]);
    for i in 0..=n {
        if i < 2 {
            is_prime[i] = false;
        }
        if is_prime[i] {
            primes.push(i);
            min_primes[i] = i;
        }
        for &p in primes.iter() {
            if i * p > n || p > min_primes[i] {
                break;
            }
            is_prime[i * p] = false;
            min_primes[i * p] = p;
        }
    }
    primes
}

/// **O(n log(log(n)))** calculate vec, which vec[i] mean min(factorization(n))
fn min_primes(size: usize) -> Vec<usize> {
    let mut sieve: Vec<_> = (0..=size).collect();
    for i in (2..).take_while(|i| i * i <= size) {
        for j in 2..=(size / i) {
            if sieve[i * j] == i * j {
                sieve[i * j] = i;
            }
        }
    }
    sieve
}

/// **O(sqrt(n))** calculate prime factorization of n
fn factorization(n: usize) -> HashMap<usize, usize> {
    if n < 2 {
        return vec![(n, 1)].into_iter().collect();
    }
    let (mut divided, mut facts) = (n, HashMap::new());
    for i in (2..).take_while(|i| i * i <= n) {
        while divided % i == 0 {
            divided /= i;
            *facts.entry(i).or_insert(0) += 1;
        }
    }
    if divided > 1 {
        *facts.entry(divided).or_insert(0) += 1;
    }
    facts
}

/// **O(log(n))** calculate prime factorization of n, with min_primes
fn factorization_with_min_primes(n: usize, min_primes: &[usize]) -> HashMap<usize, usize> {
    if n <= 1 {
        return vec![(n, 1)].into_iter().collect();
    }
    let (mut divided, mut facts) = (n, HashMap::new());
    while divided > 1 {
        *facts.entry(min_primes[divided]).or_insert(0) += 1;
        divided /= min_primes[divided];
    }
    facts
}

/// **O(sqrt(n))** calculate vec of devisors
fn devisors(n: usize) -> Vec<usize> {
    let (mut d, mut r) = (Vec::new(), Vec::new());
    for i in (1..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            d.push(i);
            if i < n / i {
                r.push(n / i);
            }
        }
    }
    d.iter().chain(r.iter().rev()).map(|&x| x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sieve_test0() {
        assert_eq!(sieve_of_eratosthenes(0), vec![false]);
    }

    #[test]
    fn sieve_test1() {
        assert_eq!(sieve_of_eratosthenes(1), vec![false, false]);
    }

    #[test]
    fn sieve_test2() {
        assert_eq!(sieve_of_eratosthenes(2), vec![false, false, true]);
    }
    #[test]
    fn sieve_test30() {
        assert_eq!(
            sieve_of_eratosthenes(30),
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
            sieve_of_eratosthenes(100),
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
        assert_eq!(primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn primes_test100() {
        assert_eq!(
            primes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn fast_sieve_test0() {
        assert_eq!(fast_primes(0), vec![]);
    }

    #[test]
    fn fast_sieve_test1() {
        assert_eq!(fast_primes(1), vec![]);
    }

    #[test]
    fn fast_sieve_test2() {
        assert_eq!(fast_primes(2), vec![2]);
    }

    #[test]
    fn fast_sieve_test30() {
        assert_eq!(fast_primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn fast_sieve_test100() {
        assert_eq!(
            fast_primes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }

    #[test]
    fn min_primes_test30() {
        assert_eq!(
            min_primes(30),
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2
            ]
        );
    }
    #[test]
    fn min_primes_test100() {
        assert_eq!(
            min_primes(100),
            vec![
                0, 1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2, 3, 2, 23, 2, 5,
                2, 3, 2, 29, 2, 31, 2, 3, 2, 5, 2, 37, 2, 3, 2, 41, 2, 43, 2, 3, 2, 47, 2, 7, 2, 3,
                2, 53, 2, 5, 2, 3, 2, 59, 2, 61, 2, 3, 2, 5, 2, 67, 2, 3, 2, 71, 2, 73, 2, 3, 2, 7,
                2, 79, 2, 3, 2, 83, 2, 5, 2, 3, 2, 89, 2, 7, 2, 3, 2, 5, 2, 97, 2, 3, 2
            ]
        );
    }
    #[test]
    fn factorization_test01() {
        assert_eq!(factorization(0), vec![(0, 1)].into_iter().collect());
        assert_eq!(factorization(1), vec![(1, 1)].into_iter().collect());
    }

    #[test]
    fn factorization_test() {
        assert_eq!(factorization(2), vec![(2, 1)].into_iter().collect());
        assert_eq!(factorization(4), vec![(2, 2)].into_iter().collect());
        assert_eq!(factorization(8), vec![(2, 3)].into_iter().collect());
        assert_eq!(factorization(16), vec![(2, 4)].into_iter().collect());
        assert_eq!(
            factorization(15),
            vec![(3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization(60),
            vec![(2, 2), (3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization(300),
            vec![(2, 2), (3, 1), (5, 2)].into_iter().collect()
        );
    }

    #[test]
    fn factorization_with_min_primes_test01() {
        let p = min_primes(100);
        assert_eq!(
            factorization_with_min_primes(0, &p),
            vec![(0, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(1, &p),
            vec![(1, 1)].into_iter().collect()
        );
    }

    #[test]
    fn factorization_with_min_primes_test() {
        let p = min_primes(300);
        assert_eq!(
            factorization_with_min_primes(2, &p),
            vec![(2, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(4, &p),
            vec![(2, 2)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(8, &p),
            vec![(2, 3)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(16, &p),
            vec![(2, 4)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(15, &p),
            vec![(3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(60, &p),
            vec![(2, 2), (3, 1), (5, 1)].into_iter().collect()
        );
        assert_eq!(
            factorization_with_min_primes(300, &p),
            vec![(2, 2), (3, 1), (5, 2)].into_iter().collect()
        );
    }

    #[test]
    fn devisors_test() {
        assert_eq!(devisors(12), [1, 2, 3, 4, 6, 12]);
        assert_eq!(devisors(25), [1, 5, 25]);
        assert_eq!(
            devisors(720),
            [
                1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 30, 36, 40, 45, 48, 60, 72, 80,
                90, 120, 144, 180, 240, 360, 720,
            ]
        )
    }

    #[test]
    fn devisors_bound_test() {
        assert_eq!(devisors(0), []);
        assert_eq!(devisors(1), [1]);
        assert_eq!(devisors(2), [1, 2]);
    }
}
