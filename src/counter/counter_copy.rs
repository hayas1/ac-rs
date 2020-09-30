#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};

trait Counter<T> {
    fn new<I: Iterator<Item = T>>(data: I) -> Self;
    fn count(&self, elem: T) -> usize;
    fn most_common(&self) -> Vec<(T, usize)>;
}
impl<T: Hash + Eq + Copy> Counter<T> for HashMap<T, usize> {
    fn new<I: Iterator<Item = T>>(data: I) -> Self {
        let mut count = HashMap::new();
        for d in data {
            *count.entry(d).or_insert(0) += 1;
        }
        count
    }

    fn count(&self, elem: T) -> usize {
        match self.get(&elem) {
            Some(&e) => e,
            None => 0,
        }
    }

    fn most_common(&self) -> Vec<(T, usize)> {
        if self.is_empty() {
            return Vec::new();
        }
        let r = 16;
        let max_digits = self
            .iter()
            .map(|(&_, &c)| {
                let (mut r_cnt, mut fx) = (1, c);
                loop {
                    if fx == 0 {
                        break r_cnt;
                    } else {
                        fx /= r;
                        r_cnt += 1;
                    }
                }
            })
            .max()
            .unwrap();

        let mut sorted: Vec<_> = self.iter().map(|(&x, &c)| (x, c)).collect();
        let mut bucket = vec![Vec::new(); r];
        for dg in 0..max_digits {
            for &dt in sorted.iter() {
                let m = dt.1 / r.pow(dg) % r;
                bucket[m].push(dt);
            }
            sorted.clear(); // warning: O(n)
            for b in bucket.iter_mut().rev() {
                sorted.extend(&*b);
                b.clear();
            }
        }
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_test() {
        let data = vec![1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 2, 3];
        let c: HashMap<_, _> = Counter::new(data.iter());
        assert_eq!(c[&1], 2);
        assert_eq!(c.count(&1), 2);
        assert_eq!(c.count(&10), 0);
    }

    #[test]
    fn most_common_test() {
        let data = "this is it";
        let c: HashMap<_, _> = Counter::new(data.chars());
        let mc = c.most_common();
        assert!(
            mc == [('i', 3), ('s', 2), ('t', 2), (' ', 2), ('h', 1)]
                || mc == [('i', 3), ('s', 2), (' ', 2), ('t', 2), ('h', 1)]
                || mc == [('i', 3), ('t', 2), ('s', 2), (' ', 2), ('h', 1)]
                || mc == [('i', 3), ('t', 2), (' ', 2), ('s', 2), ('h', 1)]
                || mc == [('i', 3), (' ', 2), ('s', 2), ('t', 2), ('h', 1)]
                || mc == [('i', 3), (' ', 2), ('t', 2), ('s', 2), ('h', 1)]
        )
    }
}
