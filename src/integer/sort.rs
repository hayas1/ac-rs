#![allow(dead_code)]

/// **O(n + (max(data)-min(data)))** return stable sorted data.
pub fn counting_sorted(data: &[usize]) -> Vec<usize> {
    if data.is_empty() {
        return Vec::new();
    }
    let (&min, &max) = (data.iter().min().unwrap(), data.iter().max().unwrap());
    let mut count = vec![0; max - min + 1];
    for &d in data {
        count[d - min] += 1;
    }
    let mut sorted = Vec::new();
    for (i, &c) in count.iter().enumerate() {
        if c > 0 {
            sorted.extend(vec![i + min; c]);
        }
    }
    sorted
}

/// **O(n + (max(f(data))-min(f(data))))** return stable sorted data.
pub fn counting_sorted_with<T, F>(data: &[T], f: F) -> Vec<&T>
where
    F: Fn(&T) -> usize,
{
    if data.is_empty() {
        return Vec::new();
    }
    let (min, max) = (
        data.iter().map(|x| f(x)).min().unwrap(),
        data.iter().map(|x| f(x)).max().unwrap(),
    );
    let mut count = vec![Vec::new(); max - min + 1];
    for d in data {
        count[f(d) - min].push(d);
    }
    let mut sorted = Vec::new();
    for c in count.iter() {
        if !c.is_empty() {
            sorted.extend(c);
        }
    }
    sorted
}

/// **O(n(log(max(f(data))))** return stable sorted data.
pub fn radix_sorted_with<T, F>(data: &[T], f: F) -> Vec<&T>
where
    F: Fn(&T) -> usize,
{
    if data.is_empty() {
        return Vec::new();
    }
    let r = 16;
    let max_digits = data
        .iter()
        .map(|x| {
            let (mut r_cnt, mut fx) = (1, f(x));
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

    let mut sorted: Vec<_> = data.iter().collect();
    let mut bucket = vec![Vec::new(); r];
    for dg in 0..max_digits {
        for &dt in sorted.iter() {
            let m = f(dt) / r.pow(dg) % r;
            bucket[m].push(dt);
        }
        sorted.clear(); // warning: O(n)
        for b in bucket.iter_mut() {
            sorted.extend(&*b);
            b.clear();
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_sort_test() {
        let v = [1, 5, 5, 4, 4, 4, 5, 1];
        assert_eq!(counting_sorted(&v), [1, 1, 4, 4, 4, 5, 5, 5]);
        assert_eq!(v, [1, 5, 5, 4, 4, 4, 5, 1]);
    }
    #[test]
    fn counting_sorted_with_test() {
        let v = [(2, "two0"), (2, "two1"), (3, "three0"), (1, "one0")];
        assert_eq!(
            counting_sorted_with(&v, |&(x, _)| x),
            [&(1, "one0"), &(2, "two0"), &(2, "two1"), &(3, "three0")]
        );
        assert_eq!(
            counting_sorted_with(&v, |&(x, _)| x),
            [&v[3], &v[0], &v[1], &v[2]]
        );
        assert_eq!(v, [(2, "two0"), (2, "two1"), (3, "three0"), (1, "one0")]);
    }

    #[test]
    fn radix_sorted_with_test() {
        let v = [(2, "two0"), (2, "two1"), (3, "three0"), (1, "one0")];
        assert_eq!(
            radix_sorted_with(&v, |&(x, _)| x),
            [&(1, "one0"), &(2, "two0"), &(2, "two1"), &(3, "three0")]
        );
        assert_eq!(
            radix_sorted_with(&v, |&(x, _)| x),
            [&v[3], &v[0], &v[1], &v[2]]
        );
        assert_eq!(v, [(2, "two0"), (2, "two1"), (3, "three0"), (1, "one0")]);
    }

    #[test]
    fn radix_sorted_test() {
        let v = [
            1, 3, 1101, 1101, 2221, 983, 1235, 6, 234, 33, 5413, 7346, 76, 12, 1123, 6532, 9999,
        ];
        assert_eq!(
            radix_sorted_with(&v, |&x| x)
                .iter()
                .map(|&&x| x)
                .collect::<Vec<_>>(),
            [1, 3, 6, 12, 33, 76, 234, 983, 1101, 1101, 1123, 1235, 2221, 5413, 6532, 7346, 9999],
        );
    }
}
