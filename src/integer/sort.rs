/// O(n + (max(data)-min(data))) # return sorted data.
fn counting_sorted(data: &[usize]) -> Vec<usize> {
    if data.len() < 1 {
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

/// O(n + (max(f(data))-min(f(data)))) # return sorted data.
fn counting_sorted_with<T, F>(data: &[T], f: F) -> Vec<&T>
where
    F: Fn(&T) -> usize,
{
    if data.len() < 1 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_sort_test() {
        let v = [1, 5, 5, 4, 4, 4, 5, 1];
        assert_eq!(counting_sorted(&v), [1, 1, 4, 4, 4, 5, 5, 5]);
    }
    #[test]
    fn counting_sort_with_test() {
        let v = [(2, "two0"), (2, "two1"), (3, "three0"), (1, "one0")];
        assert_eq!(
            counting_sorted_with(&v, |&(x, _)| x),
            [&(1, "one0"), &(2, "two0"), &(2, "two1"), &(3, "three0")]
        );
        assert_eq!(
            counting_sorted_with(&v, |&(x, _)| x),
            [&v[3], &v[0], &v[1], &v[2]]
        );
    }
}
