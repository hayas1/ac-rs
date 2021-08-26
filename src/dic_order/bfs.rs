use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

/// **O(|candidate| * (kinds of chars)^2)**, generate the kth item in dictionary order <br>
/// warning: this use factorial, so large candidate will cause overflow <br>
/// warning: too large k such as larger than all permutations will cause panic <br>
pub fn kth_dic_order<T>(candidate: &[T], k: usize) -> Vec<T>
where
    T: Clone + Hash + Eq + Ord,
{
    let factorial = (0..=candidate.len())
        .scan(1, |fac, x| {
            *fac *= x.max(1);
            Some(*fac)
        })
        .collect_vec();
    let mut counter = HashMap::new();
    for c in candidate.iter() {
        *counter.entry(c.clone()).or_insert(0) += 1;
    }
    bfs(&mut counter, k, &factorial).into_iter().collect()
}
/// **O(|candidate| * |chars|^2)**, (|chars| mean HashMap's keys size), find the kth item in dictionary order
fn bfs<T>(chars: &mut HashMap<T, usize>, k: usize, factorial: &[usize]) -> VecDeque<T>
where
    T: Clone + Hash + Eq + Ord,
{
    if k <= 1 {
        return chars.iter().sorted().map(|(c, &n)| vec![c.clone(); n]).flatten().collect();
    }
    let sorted =
        chars.iter().filter(|&(_, &n)| n > 0).map(|(c, _)| c.clone()).sorted().collect_vec();
    let permutations: Vec<_> = vec![0]
        .into_iter()
        .chain(sorted.iter().scan(0, |cum, c| {
            *cum += permutation(c.clone(), &chars, factorial);
            Some(*cum)
        }))
        .collect();
    for (c, (&p, &np)) in sorted.iter().zip(permutations.iter().tuple_windows()) {
        if p < k && k <= np {
            *chars.get_mut(c).expect("above added") -= 1;
            let mut generated = bfs(chars, k - p, factorial);
            generated.push_front(c.clone());
            return generated;
        }
    }

    unreachable!("maybe possible permutations {} < k({})", permutations[permutations.len() - 1], k);
}
/// **O(|chars|)** (|chars| mean HashMap's keys size), calculate permutation if given char is head
fn permutation<T>(head: T, chars: &HashMap<T, usize>, fac: &[usize]) -> usize
where
    T: Clone + Hash + Eq + Ord,
{
    let len = chars.values().fold(0, |sum, x| sum + x);
    if len > 1 {
        let divider = chars.iter().fold(1, |prod, (c, &b)| prod * fac[b - (c == &head) as usize]);
        fac[len - 1] / divider
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_dic_order_test1() {
        let str: Vec<_> = "aabbcc".chars().collect();
        assert_eq!(kth_dic_order(&str, 0), "aabbcc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 1), "aabbcc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 2), "aabcbc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 3), "aabccb".chars().collect::<Vec<_>>());
    }

    #[test]
    #[should_panic]
    fn kth_dic_order_test2() {
        kth_dic_order(&"aaa".chars().collect::<Vec<_>>(), 2);
    }
}
