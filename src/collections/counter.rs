use crate::integer::sort::radix_sorted_with;

use std::{collections::HashMap, hash::Hash};

pub trait Counter<T>: Sized {
    /// **O(n)**, new empty counter
    fn new() -> Self;
    /// **O(n)**, count duplicate elements data
    fn from<I: Iterator<Item = T>>(data: I) -> Self {
        let mut counter = Self::new();
        for d in data {
            counter.count(d);
        }
        counter
    }
    /// **O(1)**, count new element
    fn count(&mut self, elem: T);
    /// **O(1)**, remove element
    fn remove(&mut self, elem: &T);
    /// **O(1)**, count the number of occurrences of elem
    fn counted(&self, elem: T) -> usize;
    /// **O(n log(usize::MAX))**, get vec with sorted in descending order by count
    fn most_common(&self) -> Vec<(T, usize)>;
}
impl<T: Hash + Eq + Copy> Counter<T> for HashMap<T, usize> {
    fn new() -> Self {
        HashMap::new()
    }

    fn count(&mut self, elem: T) {
        *self.entry(elem).or_insert(0) += 1;
    }

    fn remove(&mut self, elem: &T) {
        if let Some(x) = self.get_mut(elem) {
            *x -= 1;
            if *x <= 0 {
                self.remove(elem);
            }
        }
    }

    fn counted(&self, elem: T) -> usize {
        match self.get(&elem) {
            Some(&e) => e,
            None => 0,
        }
    }

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
    fn counted_test() {
        let data = vec![1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 2, 3];
        let c: HashMap<_, _> = Counter::from(data.iter());
        assert_eq!(c[&1], 2);
        assert_eq!(c.counted(&1), 2);
        assert_eq!(c.counted(&10), 0);
    }

    #[test]
    fn most_common_test() {
        let data = "this is it";
        let c: HashMap<_, _> = Counter::from(data.chars());
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
    fn empty_counter_test() {
        let mut c: HashMap<_, _> = Counter::new();
        c.count("rust");
        c.count("rust");
        assert_eq!(c.counted("rust"), 2);
        c.count("ruby");
        assert_eq!(c.counted("rust"), 2);
        assert_eq!(c.counted("ruby"), 1);
        assert_eq!(c.counted("python"), 0);
        c.remove("ruby");
        assert_eq!(c.counted("ruby"), 0);
        assert_eq!(c.counted("rust"), 2);
    }

    #[test]
    fn normal_hashmap_test() {
        let mut hm = HashMap::new();
        hm.insert("one", 1);
        hm.insert("two", 2);
        assert_eq!(hm.counted("two"), 2); // normal hashmap has no method count, but hashmap has it in this module
    }
}
