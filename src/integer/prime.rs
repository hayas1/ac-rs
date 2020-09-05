#![allow(dead_code)]
// prime number is calculated as a index of Vec, so the type is usize

/// O(n log(log(n))) # calculate n size vec, which vec[i] mean i is prime or not, with sieve of Eratosthenes
fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut sieve: Vec<_> = (0..=n).collect();
    for i in 1.. {
        if i * i > n {
            break;
        }
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

/// O(n log(log(n))) # calculate vec of primes from 0 to max
fn primes(max: usize) -> Vec<usize> {
    sieve_of_eratosthenes(max)
        .iter()
        .enumerate()
        .filter(|&(_i, &x)| x)
        .map(|(i, _x)| i)
        .collect()
}

/// O(n)...? # calculate vec of primes from 0 to max
fn fast_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let (mut is_prime, mut spf) = (vec![true; n + 1], vec![0; n + 1]);
    for i in 0..=n {
        if i < 2 {
            is_prime[i] = false;
        }
        if is_prime[i] {
            primes.push(i);
            spf[i] = i;
        }
        for &p in primes.iter() {
            if i * p > n || p > spf[i] {
                break;
            }
            is_prime[i * p] = false;
            spf[i * p] = p;
        }
    }
    primes
}

/// O(n log(n)) # calculate vec, which vec[i] mean min(factorization(n))
fn min_primes(size: usize) -> Vec<usize> {
    let mut sieve: Vec<_> = vec![None; size + 1];
    for i in 2..=size {
        for j in 2..=(size / i) {
            if i * j % i == 0 && sieve[i * j] == None {
                sieve[i * j] = Some(i);
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .map(|(i, &x)| match x {
            Some(np) => np,
            None => i,
        })
        .collect()
}

/// O(sqrt(n)) # calculate prime factorization of n
fn factorization(n: usize) -> Vec<usize> {
    if n < 2 {
        vec![n]
    } else {
        let (mut dived, mut pf) = (n, Vec::new());
        for i in 2.. {
            if i * i > n {
                if dived > 1 {
                    pf.push(dived);
                }
                break;
            }
            while dived % i == 0 {
                dived /= i;
                pf.push(i);
            }
        }
        pf
    }
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
        assert_eq!(factorization(0), vec![0]);
        assert_eq!(factorization(1), vec![1]);
    }

    #[test]
    fn factorization_test() {
        assert_eq!(factorization(2), vec![2]);
        assert_eq!(factorization(4), vec![2, 2]);
        assert_eq!(factorization(8), vec![2, 2, 2]);
        assert_eq!(factorization(16), vec![2, 2, 2, 2]);
        assert_eq!(factorization(15), vec![3, 5]);
        assert_eq!(factorization(60), vec![2, 2, 3, 5]);
    }
}
