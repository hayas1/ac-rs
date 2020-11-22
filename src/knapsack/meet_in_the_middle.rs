#![allow(dead_code)]

use num::Integer;
use std::{collections::HashMap, hash::Hash};

/// O(2^(n/2)) # knapsack capacity is c, value of pi is v[i], weight of p_i is w[i]
fn knapsack_half_enumerate<V, W>(n: usize, c: W, w: &[W], v: &[V]) -> V
where
    W: Integer + Copy + Hash,
    V: Integer + Copy,
{
    /// O(2^(n/2)) # compute list of tuple that (weight w, max value v such as weight is at most w)
    fn build<V, W>(n2: usize, c: W, w: &[W], v: &[V]) -> Vec<(W, V)>
    where
        W: Integer + Copy + Hash,
        V: Integer + Copy,
    {
        let mut map = HashMap::new();
        for i in 1..1 << n2 {
            let weight = (0..n2).fold(
                W::zero(),
                |sum, j| if i >> j & 1 == 1 { sum + w[j] } else { sum },
            );
            if weight > c {
                break;
            }
            let value = (0..n2).fold(
                V::zero(),
                |sum, j| if i >> j & 1 == 1 { sum + v[j] } else { sum },
            );
            map.entry(weight)
                .and_modify(|e: &mut V| *e = (*e).max(value))
                .or_insert(value);
        }
        map.iter().map(|(&w, &v)| (w, v)).collect()
        // let mut map_sorted: Vec<_> = map.iter().map(|(&w, &v)| (w, v)).collect();
        // map_sorted.sort_by(|a, b| a.0.cmp(&b.0));
        // map_sorted
    }

    let (v1, v2) = v.split_at(n / 2);
    let (w1, w2) = w.split_at(n / 2);
    let (mut map1, mut map2) = (build(n / 2, c, w1, v1), build(n / 2, c, w2, v2));
    map1.sort_by(|a, b| a.0.cmp(&b.0));
    map2.sort_by(|a, b| b.0.cmp(&a.0));
    let (mut max_value, mut j) = (V::zero(), 0);
    for i in 0..map1.len() {
        // two pointer method
        let (w1, v1) = map1[i];
        while j < map2.len() && (w1 + map2[j].0) > c {
            j += 1;
        }
        if j < map2.len() {
            max_value = max_value.max(v1 + map2[j].1);
        }
    }
    max_value
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack_half_enumerate_test1() {
        let (n, c) = (4, 10);
        let (w, v) = (vec![4, 7, 2, 4], vec![1, 3, 1, 2]);
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 4);
    }

    #[test]
    fn knapsack_half_enumerate_test2() {
        let (n, c) = (6, 15);
        let w = vec![2, 1, 3, 2, 1, 5];
        let v = vec![3, 2, 6, 1, 3, 85];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 100);
    }

    #[test]
    fn knapsack_half_enumerate_test3() {
        let (n, c) = (4, 15);
        let w = vec![10, 3, 5, 7];
        let v = vec![6, 7, 2, 4];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 13);
    }
}
