/// **O(n^2)**, stable sorted by bubble sort
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

/// **O(n^2)**, stable sorted by selection sort
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

/// **O(n + inversion_number(data))**, stable sorted by insertion sort
pub fn insertion_sort<T: PartialOrd>(data: &mut [T]) {
    for i in 0..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// **O(n log(n))**, sorted by heap sort
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

/// **O(n log(n))**, stable sorted by merge sort
pub fn merge_sort<T: PartialOrd + Clone>(data: &mut [T]) {
    fn merge_sort_recursive<T: PartialOrd + Clone>(data: &mut [T], from: usize, to: usize) {
        if to - from < 2 {
            return;
        } else {
            let mid = (from + to) / 2;
            merge_sort_recursive(data, from, mid);
            merge_sort_recursive(data, mid, to);
            merge(data, from, mid, to);
        }
    }
    fn merge<T: PartialOrd + Clone>(data: &mut [T], from: usize, mid: usize, to: usize) {
        let (mut left, mut right): (Vec<_>, Vec<_>) = (
            data[from..mid].iter().cloned().collect(),
            data[mid..to].iter().cloned().collect(),
        );
        for i in (from..to).rev() {
            data[i] = match (left.last(), right.last()) {
                (Some(ll), Some(rl)) => {
                    if ll > rl {
                        left.pop()
                    } else {
                        right.pop()
                    }
                }
                (Some(_ll), None) => left.pop(),
                (None, Some(_rl)) => right.pop(),
                (None, None) => unreachable!(),
            }
            .unwrap();
        }
    }
    merge_sort_recursive(data, 0, data.len());
}

/// **O(n log(n))**, sorted data by quick sort
pub fn quick_sort<T: PartialOrd>(data: &mut [T]) {
    fn quick_sort_recursive<T: PartialOrd>(data: &mut [T], from: usize, to: usize) {
        if to - from < 2 {
            return;
        } else {
            let mut pivot_pos = (from + to) / 2;
            let (mut left, mut right) = (from, to - 1);
            while left < right {
                while data[left] < data[pivot_pos] {
                    left += 1;
                }
                while data[pivot_pos] < data[right] {
                    right -= 1;
                }
                if left == pivot_pos {
                    pivot_pos = right;
                } else if right == pivot_pos {
                    pivot_pos = left;
                }
                data.swap(left, right);
            }
            quick_sort_recursive(data, from, left);
            quick_sort_recursive(data, right, to);
        }
    }
    quick_sort_recursive(data, 0, data.len())
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
    fn merge_sort_test() {
        let mut v = vec![32, 21, 42, 12, 11, 8];
        merge_sort(&mut v);
        assert_eq!(v, vec![8, 11, 12, 21, 32, 42]);
    }

    #[test]
    fn quick_sort_test() {
        let mut v = vec![45, 12, 72, 38, 92, 4];
        quick_sort(&mut v);
        assert_eq!(v, vec![4, 12, 38, 45, 72, 92]);
    }

    #[test]
    fn is_sorted_test_n_pow_2() {
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
            let mut v = vec![0.0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            insertion_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
    }

    #[test]
    fn is_sorted_test_n_log_n() {
        use rand::Rng;
        for i in 0..100 {
            let mut v = vec![0.0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            heap_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
        for i in 0..100 {
            let mut v = vec![0.0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            merge_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
        for i in 0..100 {
            let mut v = vec![0.0; 100 * i];
            rand::thread_rng().fill(&mut v[..]);
            quick_sort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
    }

    #[test]
    fn empty_sort_test() {
        let mut v = vec![0; 0];
        bubble_sort(&mut v);
        assert_eq!(v, Vec::new());
        selection_sort(&mut v);
        assert_eq!(v, Vec::new());
        insertion_sort(&mut v);
        assert_eq!(v, Vec::new());
        heap_sort(&mut v);
        assert_eq!(v, Vec::new());
        merge_sort(&mut v);
        assert_eq!(v, Vec::new());
        quick_sort(&mut v);
        assert_eq!(v, Vec::new());
    }
}
