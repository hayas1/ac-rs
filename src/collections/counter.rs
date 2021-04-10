use crate::integer::sort::radix_sorted_with;

use std::{collections::HashMap, hash::Hash};

pub trait Counter<T> {
    fn new<I: Iterator<Item = T>>(data: I) -> Self;
    fn count(&self, elem: T) -> usize;
    fn most_common(&self) -> Vec<(T, usize)>;
}
impl<T: Hash + Eq + Copy> Counter<T> for HashMap<T, usize> {
    /// **O(n)**, count duplicate elements data
    fn new<I: Iterator<Item = T>>(data: I) -> Self {
        let mut count = HashMap::new();
        for d in data {
            *count.entry(d).or_insert(0) += 1;
        }
        count
    }

    /// **O(1)**, count the number of occurrences of elem
    fn count(&self, elem: T) -> usize {
        match self.get(&elem) {
            Some(&e) => e,
            None => 0,
        }
    }

    /// **O(n log(usize::MAX))**, get vec with sorted in descending order by count
    fn most_common(&self) -> Vec<(T, usize)> {
        radix_sorted_with(&self.iter().map(|(&x, &c)| (x, c)).collect::<Vec<_>>(), |&(_k, v)| v)
            .iter()
            .rev()
            .map(|&&x| x)
            .collect()
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

    #[test]
    fn normal_hashmap_test() {
        let mut hm = HashMap::new();
        hm.insert("one", 1);
        hm.insert("two", 2);
        assert_eq!(hm.count("two"), 2); // normal hashmap has no method count, but hashmap has it in this module
    }
}
