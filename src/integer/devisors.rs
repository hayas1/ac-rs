use num::{range_inclusive, Integer, ToPrimitive, Unsigned};

/// **O(sqrt(n))**, calculate vec of pair os devisors
pub fn devisors_pair<T: Integer + ToPrimitive + Unsigned + Copy>(n: T) -> Vec<(T, T)> {
    range_inclusive(T::one(), n)
        .take_while(|&i| i * i <= n)
        .filter(|&i| n % i == T::zero())
        .map(|i| (i, n / i))
        .collect()
}

/// **O(sqrt(n))**, calculate vec of devisors
pub fn devisors<T: Integer + ToPrimitive + Unsigned + Copy>(n: T) -> Vec<T> {
    let (d, r): (Vec<_>, Vec<_>) = devisors_pair(n).into_iter().unzip();
    let mut devisors: Vec<_> = d.into_iter().chain(r.into_iter().rev()).collect();
    devisors.dedup(); // pre-deduplicated devisor is sorted, so it can deduplicated in this way
    devisors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_devisors_pair() {
        assert_eq!(devisors_pair(12u32), [(1, 12), (2, 6), (3, 4)]);
        assert_eq!(devisors_pair(25u64), [(1, 25), (5, 5)]);
        assert_eq!(
            devisors_pair(720u128),
            [
                (1, 720),
                (2, 360),
                (3, 240),
                (4, 180),
                (5, 144),
                (6, 120),
                (8, 90),
                (9, 80),
                (10, 72),
                (12, 60),
                (15, 48),
                (16, 45),
                (18, 40),
                (20, 36),
                (24, 30),
            ]
        )
    }

    #[test]
    fn test_devisors_pair_bound() {
        assert_eq!(devisors_pair(0u32), []);
        assert_eq!(devisors_pair(1u64), [(1, 1)]);
        assert_eq!(devisors_pair(2u128), [(1, 2)]);
    }

    #[test]
    fn test_devisors() {
        assert_eq!(devisors(12u32), [1, 2, 3, 4, 6, 12]);
        assert_eq!(devisors(25u64), [1, 5, 25]);
        assert_eq!(
            devisors(720u128),
            [
                1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 30, 36, 40, 45, 48, 60, 72, 80,
                90, 120, 144, 180, 240, 360, 720,
            ]
        )
    }

    #[test]
    fn test_devisors_bound() {
        assert_eq!(devisors(0u32), []);
        assert_eq!(devisors(1u64), [1]);
        assert_eq!(devisors(2u128), [1, 2]);
    }
}
