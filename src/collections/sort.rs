/// **O(n^2)**, stable sorted data by bubble sort
pub fn bubble_sort<T: PartialOrd>(data: &mut [T]) {
    let n = data.len();
    for i in 0..n {
        for j in 1..(n - i) {
            if data[j - 1] > data[j] {
                data.swap(j - 1, j);
            }
        }
    }
}

/// **O(n^2)**, stable sorted data by selection sort
pub fn selection_sort<T: Ord>(data: &mut [T]) {
    let n = data.len();
    for i in 0..n {
        let argmax = data[..n - i]
            .iter()
            .enumerate()
            .max_by_key(|&(_i, v)| v)
            .unwrap()
            .0;
        data.swap(n - i - 1, argmax);
    }
}

/// **O(n + inversion_number(data))**, stable sorted data by insertion sort
pub fn insertion_sort<T: PartialOrd>(data: &mut [T]) {
    for i in 0..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![6, 4, 5, 1, 2, 3];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn selection_sort_test() {
        let mut v = vec![78, 15, 63, 56, 17, 42];
        selection_sort(&mut v);
        assert_eq!(v, [15, 17, 42, 56, 63, 78]);
    }

    #[test]
    fn insertion_sort_test() {
        let mut v = vec![78, 15, 63, 56, 17, 42];
        insertion_sort(&mut v);
        assert_eq!(v, [15, 17, 42, 56, 63, 78]);
    }
}
