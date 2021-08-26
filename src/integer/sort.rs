use num::cast::AsPrimitive;

/// **O(n + (max(data)-min(data)))**, return stable sorted data.
pub fn counting_sorted<T: AsPrimitive<usize> + Copy>(data: &[T]) -> Vec<T> {
    counting_sorted_with(data, |x| x.as_()).into_iter().map(|&x| x).collect()
}

/// **O(n + (max(f(data))-min(f(data))))**, return stable sorted data.
pub fn counting_sorted_with<T, F>(data: &[T], f: F) -> Vec<&T>
where
    F: Fn(&T) -> usize,
{
    if data.is_empty() {
        return Vec::new();
    }
    let (min, max) = (
        data.iter().map(|x| f(x)).min().expect("if empty, early returned"),
        data.iter().map(|x| f(x)).max().expect("if empty, early returned"),
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

/// **O(n(log(max(data)))**, return stable sorted data.
pub fn radix_sorted<T: AsPrimitive<usize> + Copy>(data: &[T]) -> Vec<T> {
    radix_sorted_with(&data, |x| x.as_()).into_iter().map(|&x| x).collect()
}

/// **O(n(log(max(f(data))))**, return stable sorted data.
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
        .map(|x| format!("{:x}", f(x)).len() as u32)
        .max()
        .expect("if empty, early returned");
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
        let v = [(2, "two0"), (3, "three0"), (1, "one0"), (2, "two1")];
        assert_eq!(
            counting_sorted_with(&v, |&(x, _)| x),
            [&(1, "one0"), &(2, "two0"), &(2, "two1"), &(3, "three0")]
        );
        assert_eq!(counting_sorted_with(&v, |&(x, _)| x), [&v[2], &v[0], &v[3], &v[1]]);
        assert_eq!(v, [(2, "two0"), (3, "three0"), (1, "one0"), (2, "two1")]);
    }

    #[test]
    fn radix_sorted_test() {
        let v =
            [1, 3, 1101, 1101, 2221, 983, 1235, 6, 234, 33, 5413, 7346, 76, 12, 1123, 6532, 9999];
        assert_eq!(
            radix_sorted(&v),
            [1, 3, 6, 12, 33, 76, 234, 983, 1101, 1101, 1123, 1235, 2221, 5413, 6532, 7346, 9999],
        );
    }

    #[test]
    fn radix_sorted_with_test() {
        let v = [(2, "two0"), (3, "three0"), (1, "one0"), (2, "two1")];
        assert_eq!(
            radix_sorted_with(&v, |&(x, _)| x),
            [&(1, "one0"), &(2, "two0"), &(2, "two1"), &(3, "three0")]
        );
        assert_eq!(radix_sorted_with(&v, |&(x, _)| x), [&v[2], &v[0], &v[3], &v[1]]);
        assert_eq!(v, [(2, "two0"), (3, "three0"), (1, "one0"), (2, "two1")]);
    }
}
