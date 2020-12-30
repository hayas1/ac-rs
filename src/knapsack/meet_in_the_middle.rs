use num::Integer;
use std::{collections::HashMap, hash::Hash};

/// **O(2^(n/2)**, knapsack capacity is c, value of pi is v[i], weight of p_i is w[i]
pub fn knapsack_half_enumerate<V, W>(n: usize, c: W, w: &[W], v: &[V]) -> V
where
    W: Integer + Copy + Hash,
    V: Integer + Copy,
{
    /// **O(2^(n/2))**, compute list of tuple that (weight w, max value v such as weight is at most w)
    fn build<V, W>(size: usize, c: W, w: &[W], v: &[V]) -> Vec<(W, V)>
    where
        W: Integer + Copy + Hash,
        V: Integer + Copy,
    {
        let mut map = HashMap::new();
        for i in 0..1 << size {
            let (weight, value) = (0..size).fold((W::zero(), V::zero()), |sum, j| {
                let (w_sum, v_sum) = sum;
                if i >> j & 1 == 1 {
                    (w_sum + w[j], v_sum + v[j])
                } else {
                    sum
                }
            });
            if weight > c {
                continue;
            }
            map.entry(weight)
                .and_modify(|e: &mut V| *e = (*e).max(value))
                .or_insert(value);
        }
        let mut v: Vec<_> = map.iter().map(|(&w, &v)| (w, v)).collect();
        v.sort_by_key(|&(wi, _vi)| wi);
        v
    }

    let (w1, w2) = w.split_at(n / 2);
    let (v1, v2) = v.split_at(n / 2);
    let (map1, map2) = (build(n / 2, c, w1, v1), build(n - n / 2, c, w2, v2));
    // TODO! this can perform not O(n^2) but O(n), however it is not bottleneck
    // let (mut max_value, mut j) = (V::zero(), map2.len() - 1);
    // for (w1, v1) in map1 {
    //     // two pointers technique (in japanese shakutori-ho)
    //     while w1 + map2[j].0 > c {
    //         j -= if j > 0 { 1 } else { 0 };
    //     }
    //     max_value = max_value.max(v1 + map2[j].1);
    // }
    // max_value
    map1.iter()
        .map(|&(w1, v1)| {
            map2.iter()
                .map(|&(w2, v2)| if w1 + w2 <= c { v1 + v2 } else { V::zero() })
                .max()
                .unwrap_or(V::zero())
        })
        .max()
        .unwrap_or(V::zero())
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

    #[test]
    fn knapsack_half_enumerate_test4() {
        let (n, c) = (3, 10);
        let w = [9, 6, 4];
        let v = [15, 10, 6];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 16);
    }

    #[test]
    fn knapsack_half_enumerate_test5() {
        let (n, c) = (2, 10);
        let w = [9, 6];
        let v = [15, 10];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 15);
    }

    #[test]
    fn knapsack_half_enumerate_test6() {
        let (n, c) = (30, 499887702usize);
        let w = [
            137274936, 989051853, 85168425, 856699603, 611065509, 22345022, 678298936, 616908153,
            28801762, 478675378, 706900574, 738510039, 135746508, 599020879, 738084616, 545330137,
            86797589, 592749599, 401229830, 523386474, 5310725, 907821957, 565237085, 730556272,
            310581512, 136966252, 132739489, 12425915, 137199296, 23505143,
        ];
        let v = [
            128990795, 575374246, 471048785, 640066776, 819841327, 704171581, 536108301, 119980848,
            117241527, 325850062, 623319578, 998395208, 475707585, 863910036, 340559411, 122579234,
            696368935, 665665204, 958833732, 371084424, 463433600, 210508742, 685281136, 619500108,
            88215377, 558193168, 475268130, 303022740, 122379996, 304092766,
        ];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 3673016420u128);
    }

    #[test]
    fn knapsack_half_enumerate_test7() {
        let (n, c) = (10, 2921usize);
        let w = [325, 845, 371, 112, 96, 960, 161, 581, 248, 22];
        let v = [
            981421680, 515936168, 17309336, 788067075, 104855562, 494541604, 32007355, 772339969,
            55112800, 98577050,
        ];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 3657162058usize);
    }

    #[test]
    fn knapsack_half_enumerate_test8() {
        let (n, c) = (10, 936447862u64);
        let w = [
            810169801, 957981784, 687140254, 932608409, 42367415, 727293784, 870916042, 685539955,
            243593312, 977358410,
        ];
        let v = [854, 691, 294, 333, 832, 642, 139, 101, 853, 369];
        assert_eq!(knapsack_half_enumerate(n, c, &w, &v), 1686u64);
    }
}
