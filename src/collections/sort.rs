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
        let argmax = data[..(n - i)]
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

/// **O(n log(n))**, sorted data by heap sort
pub fn heap_sort<T: PartialOrd>(data: &mut [T]) {
    fn swap_index<T: PartialOrd>(data: &[T], p: usize, n: usize) -> usize {
        // larger child or p: O(1)
        let (left, right) = (p * 2 + 1, p * 2 + 2);
        if left < n {
            let large = if right < n {
                if data[left] > data[right] {
                    left
                } else {
                    right
                }
            } else {
                left
            };
            if data[p] > data[large] {
                p
            } else {
                large
            }
        } else {
            p
        }
    }
    let n = data.len();
    // heapify: O(n)
    for i in (0..(n / 2)).rev() {
        let (mut j, mut next_child) = (i, swap_index(data, i, n));
        while next_child != j {
            data.swap(j, next_child);
            j = next_child;
            next_child = swap_index(data, j, n);
        }
    }
    // sort: O(n log(n))
    for i in 0..n {
        data.swap(0, n - i - 1);
        // heap reconstruction: O(log(n))
        let (mut j, mut next_child) = (0, swap_index(data, 0, n - i - 1));
        while next_child != j {
            data.swap(j, next_child);
            j = next_child;
            next_child = swap_index(data, j, n - i - 1);
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

    #[test]
    fn heap_sort_test() {
        let mut v = vec![78, 15, 63, 56, 17, 42];
        heap_sort(&mut v);
        assert_eq!(v, [15, 17, 42, 56, 63, 78]);
    }

    #[test]
    fn is_sorted_test() {
        use rand::Rng;
        for i in 0..10 {
            let mut v = vec![0.0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            bubble_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
        for i in 0..10 {
            let mut v = vec![0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            selection_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
        for i in 0..10 {
            let mut v = vec![0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            insertion_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
        for i in 0..100 {
            let mut v = vec![0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            heap_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
    }
}
