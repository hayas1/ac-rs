use itertools::Itertools;
use std::collections::HashMap;

/// **O(|candidate| * (kinds of chars)^2)**, generate the kth item in dictionary order
/// warning: this use factorial, so large candidate will cause overflow
/// warning: too large k such as larger than all permutations will cause panic
pub fn kth_dic_order(candidate: &str, k: usize) -> String {
    let factorial = (0..=candidate.len())
        .scan(1, |fac, x| {
            *fac *= x.max(1);
            Some(*fac)
        })
        .collect_vec();
    let mut counter = HashMap::new();
    for c in candidate.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    bfs(&mut counter, k, &factorial)
}
/// **O(|candidate| * |chars|^2)**, (|chars| mean HashMap's keys size), find the kth item in dictionary order
fn bfs(chars: &mut HashMap<char, usize>, k: usize, factorial: &[usize]) -> String {
    if k <= 1 {
        return chars
            .iter()
            .sorted()
            .map(|(&c, &n)| vec![c; n].iter().collect::<String>())
            .join("");
    }
    let sorted = chars.iter().filter(|&(_, &n)| n > 0).map(|(&c, _)| c).sorted().collect_vec();
    let permutations: Vec<_> = vec![0]
        .into_iter()
        .chain(sorted.iter().scan(0, |cum, &c| {
            *cum += permutation(c, &chars, factorial);
            Some(*cum)
        }))
        .collect();
    for (c, (&p, &np)) in sorted.iter().zip(permutations.iter().tuple_windows()) {
        if p < k && k <= np {
            *chars.get_mut(c).expect("above added") -= 1;
            return c.to_string() + &bfs(chars, k - p, factorial);
        }
    }

    unreachable!("maybe possible permutations {} < k({})", permutations[permutations.len() - 1], k);
}
/// **O(|chars|)** (|chars| mean HashMap's keys size), calculate permutation if given char is head
pub fn permutation(head: char, chars: &HashMap<char, usize>, fac: &[usize]) -> usize {
    let len = chars.values().fold(0, |sum, x| sum + x);
    if len > 1 {
        let divider = chars.iter().fold(1, |prod, (&c, &b)| prod * fac[b - (c == head) as usize]);
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
        let str = "aabbcc";
        assert_eq!(kth_dic_order(str, 0), "aabbcc");
        assert_eq!(kth_dic_order(str, 1), "aabbcc");
        assert_eq!(kth_dic_order(str, 2), "aabcbc");
        assert_eq!(kth_dic_order(str, 3), "aabccb");
    }

    #[test]
    #[should_panic]
    fn kth_dic_order_test2() {
        kth_dic_order("aaa", 2);
    }
}
