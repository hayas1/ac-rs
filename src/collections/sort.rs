/// **O(n^2)**, return stable sorted data by bubble sort.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sorted_test() {
        let mut v = vec![6, 4, 5, 1, 2, 3];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }
}
