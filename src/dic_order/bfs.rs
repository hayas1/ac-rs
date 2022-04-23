use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

/// **O(|candidate| * (kinds of chars)^2)**, generate the kth item in dictionary order <br>
/// warning: this use factorial, so large candidate will cause overflow <br>
/// warning: too large k such as larger than all permutations will cause panic <br>
pub fn kth_dic_order<T>(candidate: &[T], k: usize) -> Vec<T>
where
    T: Clone + Hash + Eq + Ord,
{
    let (factorial, mut counter) = (
        (0..=candidate.len())
            .scan(1, |fac, x| {
                *fac *= x.max(1);
                Some(*fac)
            })
            .collect_vec(),
        candidate.iter().cloned().counts(),
    );
    let mut result = Vec::new();
    bfs(&mut counter, k, &factorial, &mut result);
    result
}
/// **O(|candidate| * |chars|^2)**, (|chars| mean HashMap's keys size), find the kth item in dictionary order
fn bfs<T>(chars: &mut HashMap<T, usize>, k: usize, factorial: &[usize], result: &mut Vec<T>)
where
    T: Clone + Hash + Eq + Ord,
{
    if k <= 1 {
        return result.extend(
            chars.iter().sorted().map(|(c, &n)| vec![c.clone(); n]).flatten().collect_vec(),
        );
    }
    let sorted =
        chars.iter().filter(|&(_, &n)| n > 0).map(|(c, _)| c.clone()).sorted().collect_vec();
    let permutations = vec![0]
        .into_iter()
        .chain(sorted.iter().scan(0, |cum, c| {
            *cum += permutation(c.clone(), &chars, factorial);
            Some(*cum)
        }))
        .collect_vec();
    for (c, (&p, &np)) in sorted.iter().zip(permutations.iter().tuple_windows()) {
        if p < k && k <= np {
            *chars.get_mut(c).expect("above added") -= 1;
            result.push(c.clone());
            return bfs(chars, k - p, factorial, result);
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
    if len < 2 {
        0
    } else {
        let divider = chars.iter().fold(1, |prod, (c, &b)| prod * fac[b - (c == &head) as usize]);
        fac[len - 1] / divider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_dic_order1() {
        let str: Vec<_> = "aabbcc".chars().collect();
        assert_eq!(kth_dic_order(&str, 0), "aabbcc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 1), "aabbcc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 2), "aabcbc".chars().collect::<Vec<_>>());
        assert_eq!(kth_dic_order(&str, 3), "aabccb".chars().collect::<Vec<_>>());
    }

    #[test]
    #[should_panic]
    fn test_kth_dic_order2() {
        kth_dic_order(&"aaa".chars().collect::<Vec<_>>(), 2);
    }
}
