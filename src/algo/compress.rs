/// **O(n log(n))**, return compressed vector
pub fn coordinate_compress<T: Ord + Copy>(v: &[T]) -> Vec<usize> {
    let mut vv: Vec<_> = v.iter().cloned().collect();
    vv.sort();
    vv.dedup();
    v.iter()
        .map(|x| vv.binary_search(x).expect("deduplicated vec must include all original values"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compress_test() {
        assert_eq!(coordinate_compress(&[100, 32, 4, 6]), [3, 2, 0, 1]);
        assert_eq!(coordinate_compress(&["rust", "python", "ruby", "perl"]), [3, 1, 2, 0]);
    }
}
